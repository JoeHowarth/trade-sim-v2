pub mod tabular;

use serde::de::DeserializeOwned;
use simulation::prelude::*;
use std::path::PathBuf;
use tabular::extract_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Opts {
    pub ticks: u32,
}

pub fn simulation_loop(
    opts: Opts,
    mut history: History,
) -> Result<History> {
    for _ in 0..opts.ticks {
        info!("Tick {}", history.state().tick);
        history.step()?;
    }
    Ok(history)
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
    save_json_file(tabular_path, extract_json(history)?)
}
