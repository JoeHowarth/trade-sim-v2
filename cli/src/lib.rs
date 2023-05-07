pub mod tabular;

use serde::de::DeserializeOwned;
use simulation::prelude::*;
use std::path::PathBuf;
use tabular::tabularize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Opts {
    pub ticks: u32,
}

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
        }
    }
}

pub fn run(input: InputFormat) -> Result<History> {
    let opts = input.opts.clone();
    let mut history = input.into();
    simulation_loop(opts, &mut history)?;
    Ok(history)
}

pub fn simulation_loop(
    opts: Opts,
    history: &mut History,
) -> Result<()> {
    for _ in 0..opts.ticks {
        info!("Tick {}", history.state().tick);
        history.step()?;
    }
    Ok(())
}

pub fn load_json_file<T: DeserializeOwned>(
    path: impl Into<PathBuf>,
) -> Result<T> {
    serde_json::from_reader(std::io::BufReader::new(
        std::fs::File::open(path.into())?,
    ))
    .map_err(Into::into)
}

pub fn save_json_file(
    path: impl Into<PathBuf>,
    json: impl Serialize,
) -> Result<()> {
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
