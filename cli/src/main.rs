#![allow(unused)]

use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use cli::{
    extract_actions_json, extract_agents_json, extract_markets_json,
    load_json_file, save_json_file,
};
use simulation::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(default_value_t = String::from("input/basic.json"))]
        input: String,
        #[arg(default_value_t = String::from("output/last_run.json"))]
        output: String,
        #[arg(default_value_t = String::from("output/last_run_tabular.json"))]
        tabular: String,
    },
    Resume {
        #[arg(default_value_t = String::from("output/last_run.json"))]
        prev_output: String,
        #[arg(default_value_t = String::from("output/last_run.json"))]
        output: String,
        #[arg(default_value_t = String::from("output/last_run_tabular.json"))]
        tabular: String,
        #[arg(short, long)]
        additional_ticks: u32,
    },
    Tabular,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Commands::Run {
            input,
            output,
            tabular,
        } => run(input, output, tabular),
        Commands::Resume {
            prev_output,
            output,
            tabular,
            additional_ticks,
        } => resume(prev_output, output, tabular, additional_ticks),
        Commands::Tabular => {
            let history = load_json_file("output/last_run.json")?;

            let agents = extract_agents_json(&history)?;
            let markets = extract_markets_json(&history)?;
            let actions = extract_actions_json(&history)?;

            save_json_file(
                "output/tabular/last_run.json",
                &ht_map!["agents" => agents, "markets" => markets, "actions" => actions],
            )
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

fn run(
    input: String,
    output_path: String,
    tabular_path: String,
) -> Result<()> {
    // load input file
    let InputFormat {
        opts,
        edges,
        agents,
        ports,
    } = load_json_file(input)?;

    // construct history
    let mut history = History {
        static_info: StaticInfo::new_static(&edges),
        states: vec![State {
            tick: 0,
            ports,
            agents,
        }],
        actions: vec![],
    };

    // run simulation loop
    let history = simulation_loop(opts, history)?;

    // write output file
    save_output(&history, output_path, tabular_path)
}

fn resume(
    prev_output: String,
    history_path: String,
    tabular_path: String,
    additional_ticks: u32,
) -> Result<()> {
    // load existing history to resume at
    let history = load_json_file(prev_output)?;

    // run simulation loop
    let history = simulation_loop(
        Opts {
            ticks: additional_ticks,
        },
        history,
    )?;

    save_output(&history, history_path, tabular_path)
}

fn save_output(
    history: &History,
    history_path: impl Into<PathBuf>,
    tabular_path: impl Into<PathBuf>,
) -> Result<()> {
    let agents = extract_agents_json(&history)?;
    let markets = extract_markets_json(&history)?;
    let actions = extract_actions_json(&history)?;

    save_json_file(history_path, history)?;
    save_json_file(
        tabular_path,
        &ht_map!["agents" => agents, "markets" => markets, "actions" => actions],
    )
}
