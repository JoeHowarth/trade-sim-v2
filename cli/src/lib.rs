pub mod tabular;

use serde::de::DeserializeOwned;
use simulation::{prelude::*, simulation_loop, Opts};
use std::path::PathBuf;
use tabular::tabularize;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputFormat {
    pub opts: Opts,
    pub edges: Vec<(PortId, PortId)>,
    pub agents: Vec<Agent>,
    pub ports: Vec<Port>,
}

impl Into<History> for InputFormat {
    fn into(self) -> History {
        History {
            static_info: StaticInfo::new_static(&self.edges),
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

pub fn run(input: InputFormat) -> Result<History> {
    let opts = input.opts.clone();
    let mut history = input.into();
    simulation_loop(opts, &mut history)?;
    Ok(history)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrashReport {
    pub history: History,
    #[serde(flatten)]
    pub error: SimulationError,
}

impl CrashReport {
    pub fn save(history: &History, error: Report, path: &str) -> Report {
        match error.downcast_ref::<SimulationError>() {
            Some(sim_err) => {
                let crash = CrashReport {
                    history: history.clone(),
                    error: sim_err.clone(),
                };
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

pub fn save_output(
    history: &History,
    history_path: impl Into<PathBuf>,
    tabular_path: impl Into<PathBuf>,
) -> Result<()> {
    save_json_file(history_path, history)?;
    save_json_file(tabular_path, tabularize(history)?)
}
