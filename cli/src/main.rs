#![allow(unused)]

use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use simulation::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(default_value_t = String::from("basic.json"))]
        input: String,
        #[arg(default_value_t = String::from("last_run.json"))]
        output: String,
    },
    Resume {
        #[arg(default_value_t = String::from("last_run.json"))]
        prev_output: String,
        #[arg(default_value_t = String::from("last_run.json"))]
        output: String,
        #[arg(short, long)]
        additional_ticks: u32,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Commands::Run { input, output } => {
            // load input file
            let InputFormat {
                opts,
                edges,
                agents,
                ports,
            } = serde_json::from_reader(std::io::BufReader::new(
                std::fs::File::open(
                    std::path::PathBuf::from("input").join(input),
                )?,
            ))?;

            // construct history
            let mut history = History {
                static_info: StaticInfo::new_static(&edges),
                states: vec![State {
                    tick: 0,
                    ports,
                    agents,
                }],
            };

            // run simulation loop
            let history = simulation_loop(opts, history)?;

            // write output file
            serde_json::to_writer_pretty(
                std::io::BufWriter::new(std::fs::File::create(
                    std::path::Path::new("output").join(output),
                )?),
                &history,
            )?;
            Ok(())
        }
        Commands::Resume {
            prev_output,
            output,
            additional_ticks,
        } => {
            // load input file
            let history = serde_json::from_reader(
                std::io::BufReader::new(std::fs::File::open(
                    std::path::Path::new("output").join(prev_output),
                )?),
            )?;

            // run simulation loop
            let history = simulation_loop(
                Opts {
                    ticks: additional_ticks,
                },
                history,
            )?;

            // write output file
            serde_json::to_writer_pretty(
                std::io::BufWriter::new(std::fs::File::create(
                    std::path::Path::new("output").join(output),
                )?),
                &history,
            )?;
            Ok(())
        }
    }
}

fn simulation_loop(
    opts: Opts,
    mut history: History,
) -> Result<History> {
    for _ in 0..opts.ticks {
        info!("Tick {}", history.state().tick);
        history.step()?;
    }
    Ok(history)
}

#[derive(Debug, Serialize, Deserialize)]
struct InputFormat {
    pub opts: Opts,
    pub edges: Vec<(PortId, PortId)>,
    pub agents: HTMap<AgentId, Agent>,
    pub ports: HTMap<PortId, Port>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Opts {
    ticks: u32,
}