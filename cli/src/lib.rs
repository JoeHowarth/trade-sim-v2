pub mod history;
pub mod stdio_json;
pub mod tabular;

use history::History;
use serde::de::DeserializeOwned;
use simulation::{prelude::*, run_tick, Opts, TickOutput};
use std::path::PathBuf;
use tabular::tabularize_history;

#[derive(Debug, Serialize, Deserialize)]
enum InputMsg {
    Advance,
}

pub fn simulation_loop(
    opts: Opts,
    mut ctx: Context,
    mut recorder: impl FnMut(TickOutput),
) -> Result<()> {
    for _ in 0..opts.ticks {
        info!("Tick: {}", ctx.state.tick);
        let output = run_tick(ctx)?;
        ctx = output.ctx.clone();
        recorder(output);
        std::thread::sleep(std::time::Duration::from_millis(1500))
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputFormat {
    pub opts: Opts,
    pub edges: Vec<(PortId, PortId)>,
    pub agents: Vec<Agent>,
    pub ports: Vec<Port>,
}

impl InputFormat {
    pub fn into_history(self, name: String) -> History {
        History {
            static_info: StaticInfo::new_static(name, &self.edges),
            states: vec![State {
                tick: 0,
                ports: self
                    .ports
                    .into_iter()
                    .map(|p| (p.id, p))
                    .collect(),
                agents: self
                    .agents
                    .into_iter()
                    .map(|p| (p.id, p))
                    .collect(),
            }],
            actions: vec![],
            events: vec![],
        }
    }
}

pub fn run(input: InputFormat, name: String) -> Result<History> {
    let opts = input.opts.clone();
    let mut history: History = input.into_history(name);
    let ctx = Context {
        state: history.state().clone(),
        static_info: history.static_info,
    };

    simulation_loop(opts, ctx, |tick_output| history.update(tick_output))?;
    Ok(history)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrashReport {
    pub history: History,
    #[serde(flatten)]
    pub error: SimulationError,
}

impl CrashReport {
    pub fn save(history: &History, error: Report, save_dir: &str, name: &str) -> Report {
        match error.downcast_ref::<SimulationError>() {
            Some(sim_err) => {
                let crash = CrashReport {
                    history: history.clone(),
                    error: sim_err.clone(),
                };
                let mut path = PathBuf::from(save_dir);
                path.push(format!("output/crash_report_{name}.json"));
                if let Err(file_err) = save_json_file(path, crash) {
                    error!("Didn't save file correctly");
                    return error.wrap_err(file_err);
                }
                error
            }
            None => error.wrap_err("Expected SimulationError"),
        }
    }
}

pub fn load_input_file(save_dir: &str, name: &str) -> Result<InputFormat> {
    let mut path = PathBuf::from(save_dir);
    path.push("input");
    path.push(name.to_string() + ".json");

    load_json_file(path)
}

pub fn load_json_file<T: DeserializeOwned>(path: impl Into<PathBuf>) -> Result<T> {
    let path: PathBuf = path.into();
    serde_json::from_reader(std::io::BufReader::new(
        std::fs::File::open(path.clone()).wrap_err_with(|| {
            format!(
                "File not found: {}",
                path.clone().to_string_lossy().to_string()
            )
        })?,
    ))
    .map_err(Into::into)
}

pub fn save_json_file(path: impl Into<PathBuf>, json: impl Serialize) -> Result<()> {
    serde_json::to_writer_pretty(
        std::io::BufWriter::new(std::fs::File::create(path.into())?),
        &json,
    )?;
    Ok(())
}

pub fn save_output(history: &History, save_dir: String) -> Result<()> {
    let name = history.static_info.name.clone();
    let mut path = PathBuf::from(save_dir);
    path.push("output");

    let mut history_path = path.clone();
    history_path.push(name.clone() + ".json");
    save_json_file(history_path, history)?;

    let mut tabular_path = path.clone();
    tabular_path.push(name.clone() + "_tabular.json");
    save_json_file(tabular_path, tabularize_history(history)?)?;

    // Save with name "last" too
    let mut history_path = path.clone();
    history_path.push("last.json");
    save_json_file(history_path, history)?;

    let mut tabular_path = path.clone();
    tabular_path.push("last_tabular.json");
    save_json_file(tabular_path, tabularize_history(history)?)
}
