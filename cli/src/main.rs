#![allow(unused)]

use std::{
    fs::File,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand, ValueEnum};
use cli::{
    history::History,
    load_input_file, load_json_file, save_json_file, save_output, simulation_loop,
    stdio_json::{stdout_channel, tick_output_channel},
    tabular::tabularize_history,
    CrashReport, InputFormat,
};
use log::LevelFilter;
use serde_json::json;
use simplelog::{CombinedLogger, SharedLogger, TermLogger};
use simulation::{apply_actions, prelude::*, update_world_systems, Opts};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    // #[command(flatten)]
    // config: Config,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(ValueEnum, Debug, Clone, PartialEq, Eq)]
enum StdoutBehavior {
    Log,
    PushTickOutput,
    None,
}

#[derive(Parser, Debug, Clone)]
struct Config {
    /// Whether to log to stdout or not
    #[arg(long, value_enum, default_value_t = StdoutBehavior::Log)]
    stdout_behavior: StdoutBehavior,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(long, default_value_t = String::from("./"))]
        save_dir: String,
        #[arg(long, default_value_t = String::from("last"))]
        input_name: String,
        #[arg(long)]
        output_name: Option<String>,

        #[command(flatten)]
        config: Config,
    },
    // Resume {
    //     #[arg(long, default_value_t = String::from("input/last.json"))]
    //     prev_output: String,
    //     #[arg(long, default_value_t = String::from("output/last.json"))]
    //     output: String,
    //     #[arg(long, default_value_t = String::from("output/last_tabular.json"))]
    //     tabular: String,
    //     #[arg(long, default_value_t = String::from("output/crash_report.json"))]
    //     crash_report: String,
    //     #[arg(short, long)]
    //     additional_ticks: u32,

    //     #[command(flatten)]
    //     config: Config,
    // },
    ResumeCrash {
        #[arg(default_value_t = String::from("output/crash_report.json"))]
        crash_report: String,

        #[command(flatten)]
        config: Config,
    },
    Tabular,
}

fn get_config(cli: &Cli) -> Config {
    match &cli.command {
        // Some(Commands::Resume { config, .. }) => config.clone(),
        Some(Commands::Run { config, .. }) => config.clone(),
        Some(Commands::ResumeCrash { config, .. }) => config.clone(),
        _ => Config {
            stdout_behavior: StdoutBehavior::Log,
        },
        None => todo!(),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![simplelog::WriteLogger::new(
        log::LevelFilter::Debug,
        simplelog::Config::default(),
        std::fs::File::create("output/app.log").unwrap(),
    )];
    if (get_config(&cli).stdout_behavior == StdoutBehavior::Log) {
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
            save_dir,
            input_name,
            output_name,
            config,
            ..
        }) => run(
            save_dir,
            input_name,
            output_name,
            config.stdout_behavior == StdoutBehavior::PushTickOutput,
        ),
        // Some(Commands::Resume {
        //     prev_output,
        //     output,
        //     tabular,
        //     crash_report,
        //     additional_ticks,
        //     ..
        // }) => resume(prev_output, output, tabular, crash_report, additional_ticks),
        Some(Commands::ResumeCrash { crash_report, .. }) => resume_crash(crash_report),
        Some(Commands::Tabular) => {
            let history = load_json_file("output/last.json")?;

            save_json_file("output/tabular/last.json", tabularize_history(&history)?)
        }
        None => run("./".into(), "last".into(), None, false),
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
    save_dir: String,
    input_name: String,
    output_name: Option<String>,
    push_tick_output: bool,
) -> Result<()> {
    let InputFormat {
        opts,
        edges,
        agents,
        ports,
    } = load_input_file(&save_dir, &input_name)?;
    let name = output_name.unwrap_or(input_name);

    let mut history = History {
        static_info: StaticInfo::new_static(name.clone(), &edges),
        states: vec![State {
            tick: 0,
            ports: ports.into_iter().map(|p| (p.id, p)).collect(),
            agents: agents.into_iter().map(|p| (p.id, p)).collect(),
        }],
        actions: vec![],
        events: vec![],
    };

    let ctx = Context {
        state: history.state().clone(),
        static_info: history.static_info,
    };

    let (sender, handle) = tick_output_channel();

    {
        simulation_loop(opts, ctx, |tick_output| {
            history.update(tick_output.clone());
            if push_tick_output {
                sender.send(tick_output);
            }
        })
        .map_err(|e| CrashReport::save(&history, e, &save_dir, &name))?;
        let sender = sender;
    }

    save_output(&history, save_dir)?;
    handle.join();
    Ok(())
}

// resume a simulation from `prev_output` file and continue for `additional_ticks`
// then save new output
// fn resume(
//     prev_output: String,
//     history_path: String,
//     tabular_path: String,
//     crash_report_path: String,
//     additional_ticks: u32,
// ) -> Result<()> {
//     let mut history: History = load_json_file(prev_output)?;

//     let ctx = Context {
//         state: history.state().clone(),
//         static_info: history.static_info,
//     };

//     simulation_loop(
//         Opts {
//             ticks: additional_ticks,
//         },
//         ctx,
//         |tick_output| {
//             history.update(tick_output);
//         },
//     )
//     .map_err(|e| CrashReport::save(&history, e, &save_dir, &input_name))?;

//     save_output(&history, history_path, tabular_path)
// }
