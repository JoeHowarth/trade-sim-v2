#![allow(unused)]

use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use cli::{
    load_json_file, save_json_file, save_output, simulation_loop, tabular::tabularize, CrashReport,
    InputFormat, Opts,
};
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
        #[arg(default_value_t = String::from("input/basic.json"))]
        input: String,
        #[arg(default_value_t = String::from("output/last_run.json"))]
        output: String,
        #[arg(default_value_t = String::from("output/last_run_tabular.json"))]
        tabular: String,
        #[arg(default_value_t = String::from("output/crash_report.json"))]
        crash_report: String,
    },
    Resume {
        #[arg(default_value_t = String::from("output/last_run.json"))]
        prev_output: String,
        #[arg(default_value_t = String::from("output/last_run.json"))]
        output: String,
        #[arg(default_value_t = String::from("output/last_run_tabular.json"))]
        tabular: String,
        #[arg(default_value_t = String::from("output/crash_report.json"))]
        crash_report: String,
        #[arg(short, long)]
        additional_ticks: u32,
    },
    ResumeCrash {
        #[arg(default_value_t = String::from("output/crash_report.json"))]
        crash_report: String,
    },
    Tabular,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug")
    }
    env_logger::init();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Commands::Run {
            input,
            output,
            tabular,
            crash_report,
        } => run(input, output, tabular, crash_report),
        Commands::Resume {
            prev_output,
            output,
            tabular,
            crash_report,
            additional_ticks,
        } => resume(prev_output, output, tabular, crash_report, additional_ticks),
        Commands::ResumeCrash { crash_report } => resume_crash(crash_report),
        Commands::Tabular => {
            let history = load_json_file("output/last_run.json")?;

            save_json_file("output/tabular/last_run.json", tabularize(&history)?)
        }
    }
}

fn resume_crash(crash_report: String) -> Result<()> {
    let crash_report: CrashReport = load_json_file(crash_report)?;
    let SimulationError {
        state,
        unapplied_actions,
        ..
    } = crash_report.error;
    let history = crash_report.history;

    let mut ctx = Context {
        state,
        static_info: history.static_info,
    };

    ctx = ctx.apply_actions(&unapplied_actions)?;

    // non-agent world processes
    ctx = ctx.update_world_systems();

    Ok(())
}

/// run a new simulation from the given `input` file
/// then save output
fn run(
    input: String,
    output_path: String,
    tabular_path: String,
    crash_report_path: String,
) -> Result<()> {
    let InputFormat {
        opts,
        edges,
        agents,
        ports,
    } = load_json_file(input)?;

    let mut history = History {
        static_info: StaticInfo::new_static(&edges),
        states: vec![State {
            tick: 0,
            ports: ports.into_iter().map(|p| (p.id, p)).collect(),
            agents: agents.into_iter().map(|p| (p.id, p)).collect(),
        }],
        actions: vec![],
    };

    simulation_loop(opts, &mut history)
        .map_err(|e| CrashReport::save(&history, e, &crash_report_path))?;

    save_output(&history, output_path, tabular_path)
}

/// resume a simulation from `prev_output` file and continue for `additional_ticks`
/// then save new output
fn resume(
    prev_output: String,
    history_path: String,
    tabular_path: String,
    crash_report_path: String,
    additional_ticks: u32,
) -> Result<()> {
    let mut history = load_json_file(prev_output)?;

    simulation_loop(
        Opts {
            ticks: additional_ticks,
        },
        &mut history,
    )?;

    save_output(&history, history_path, tabular_path)
}
