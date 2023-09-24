#![allow(unused)]

use std::{
    fs::File,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use cli::{
    load_json_file, save_json_file, save_output, tabular::tabularize, CrashReport, InputFormat,
};
use log::LevelFilter;
use simplelog::{CombinedLogger, Config, SharedLogger, TermLogger};
use simulation::{apply_actions, prelude::*, simulation_loop, update_world_systems, Opts};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(flatten)]
    logging: Option<LoggingConfig>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser, Debug, Clone)]
struct LoggingConfig {
    /// Whether to log to stdout or not
    #[arg(long)]
    no_log_to_term: bool,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(long, default_value_t = String::from("input/last.json"))]
        input: String,
        #[arg(long, default_value_t = String::from("output/last.json"))]
        output: String,
        #[arg(long, default_value_t = String::from("output/last_tabular.json"))]
        tabular: String,
        #[arg(long, default_value_t = String::from("output/crash_report.json"))]
        crash_report: String,

        #[command(flatten)]
        logging: Option<LoggingConfig>,
    },
    Resume {
        #[arg(long, default_value_t = String::from("input/last.json"))]
        prev_output: String,
        #[arg(long, default_value_t = String::from("output/last.json"))]
        output: String,
        #[arg(long, default_value_t = String::from("output/last_tabular.json"))]
        tabular: String,
        #[arg(long, default_value_t = String::from("output/crash_report.json"))]
        crash_report: String,
        #[arg(short, long)]
        additional_ticks: u32,

        #[command(flatten)]
        logging: Option<LoggingConfig>,
    },
    ResumeCrash {
        #[arg(default_value_t = String::from("output/crash_report.json"))]
        crash_report: String,

        #[command(flatten)]
        logging: Option<LoggingConfig>,
    },
    Tabular,
}

fn get_logging_config(cli: &Cli) -> LoggingConfig {
    match &cli.logging {
        Some(logging) => logging.clone(),
        None => match &cli.command {
            Some(Commands::Resume {
                logging: Some(logging),
                ..
            }) => logging.clone(),
            Some(Commands::Run {
                logging: Some(logging),
                ..
            }) => logging.clone(),
            Some(Commands::ResumeCrash {
                logging: Some(logging),
                ..
            }) => logging.clone(),
            _ => LoggingConfig { no_log_to_term: false },
        },
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![simplelog::WriteLogger::new(
        log::LevelFilter::Debug,
        simplelog::Config::default(),
        std::fs::File::create("output/app.log").unwrap(),
    )];
    if (!get_logging_config(&cli).no_log_to_term) {
        println!("log_to_term set");
        loggers.push(simplelog::TermLogger::new(
            log::LevelFilter::Debug,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ));
    }
    CombinedLogger::init(loggers)?;

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Some(Commands::Run {
            input,
            output,
            tabular,
            crash_report,
            ..
        }) => run(input, output, tabular, crash_report),
        Some(Commands::Resume {
            prev_output,
            output,
            tabular,
            crash_report,
            additional_ticks,
            ..
        }) => resume(prev_output, output, tabular, crash_report, additional_ticks),
        Some(Commands::ResumeCrash { crash_report, .. }) => resume_crash(crash_report),
        Some(Commands::Tabular) => {
            let history = load_json_file("output/last.json")?;

            save_json_file("output/tabular/last.json", tabularize(&history)?)
        }
        None => run(
            "input/last.json".into(),
            "output/last.json".into(),
            "output/last_tabular.json".into(),
            "output/crash_report.json".into(),
        ),
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

    let events = apply_actions(&mut ctx, &unapplied_actions)?;

    // non-agent world processes
    update_world_systems(&mut ctx);

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
        events: vec![],
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
