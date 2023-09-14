#![allow(unused_imports)]

use cli::{tabular::tabularize, CrashReport, InputFormat};
use pyo3::{
    exceptions::{self, PyValueError},
    prelude::*,
};
use pythonize::{depythonize, pythonize};
use simulation::{history::History, prelude::*, Opts, simulation_loop};

pub struct InputFormatPy(pub InputFormat);
pub struct OptsPy(pub Opts);

impl<'source> FromPyObject<'source> for InputFormatPy {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        Ok(Self(depythonize(ob)?))
    }
}

impl<'de> FromPyObject<'de> for OptsPy {
    fn extract(ob: &'de PyAny) -> PyResult<Self> {
        Ok(Self(depythonize(ob)?))
    }
}

#[pyfunction]
fn run(input: InputFormatPy) -> Result<HistoryPy> {
    cli::run(input.0).map(Into::into)
}

#[pyclass(name = "History")]
pub struct HistoryPy {
    pub history: History,
    pub crash_report_path: String,
}

impl From<History> for HistoryPy {
    fn from(history: History) -> Self {
        HistoryPy {
            history,
            crash_report_path: "../../output/crash_report.json".to_string(),
        }
    }
}

#[pymethods]
impl HistoryPy {
    #[new]
    pub fn new(history: &PyAny) -> Result<HistoryPy> {
        Ok(depythonize::<History>(history)?.into())
    }

    #[staticmethod]
    pub fn from_input(input: InputFormatPy) -> Result<HistoryPy> {
        let history: History = input.0.into();
        Ok(history.into())
    }

    pub fn run(&mut self, opts: OptsPy) -> Result<()> {
        simulation_loop(opts.0, &mut self.history)
            .map_err(|e| CrashReport::save(&self.history, e, &self.crash_report_path))
    }

    #[getter]
    pub fn history(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| pythonize(py, &self.history)).map_err(Into::into)
    }

    pub fn tabular(&self) -> PyResult<PyObject> {
        let tabular = tabularize(&self.history)?;
        Python::with_gil(|py| pythonize(py, &tabular)).map_err(Into::into)
    }
}

#[pyfunction]
pub fn bar() -> String {
    error!("bad 2");
    sus();
    "hi".into()
}

fn sus() {
    error!("sus 2");
}

/// A Python module implemented in Rust.
#[pymodule]
fn simrs(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(bar, m)?)?;
    m.add_class::<HistoryPy>()?;
    Ok(())
}
