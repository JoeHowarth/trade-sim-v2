#![allow(unused_imports)]

use cli::{simulation_loop, tabular::tabularize, InputFormat, Opts};
use pyo3::{
    exceptions::{self, PyValueError},
    prelude::*,
};
use pythonize::{depythonize, pythonize};
use simulation::{history::History, prelude::*};

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
    pub err: Option<SimulationError>,
}

impl From<History> for HistoryPy {
    fn from(history: History) -> Self {
        HistoryPy { history, err: None }
    }
}

#[pymethods]
impl HistoryPy {
    #[new]
    pub fn new(ob: &PyAny) -> Result<HistoryPy> {
        Ok(depythonize::<History>(ob)?.into())
    }

    #[staticmethod]
    pub fn from_input(input: InputFormatPy) -> Result<HistoryPy> {
        let history: History = input.0.into();
        Ok(history.into())
    }

    pub fn run(&mut self, opts: OptsPy) -> Result<()> {
        for _ in 0..opts.0.ticks {
            info!("Tick {}", self.history.state().tick);
            if let Err(err) = self.history.step() {
                self.err =
                    err.downcast_ref::<SimulationError>().cloned();
                Err(err)?;
            }
        }
        Ok(())
    }

    pub fn error(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| pythonize(py, &self.err))
            .map_err(Into::into)
    }

    // pub fn invalid_action(&self) -> PyResult<PyObject> {
    //     let Some(err) = self.err else {
    //         return Python::with_gil(|py| Ok(py.None()))
    //     };
    //     todo!()
    //     // let err =

    //     // Python::with_gil(|py| pythonize(py, ))
    // }

    #[getter]
    pub fn history(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| pythonize(py, &self.history))
            .map_err(Into::into)
    }

    pub fn tabular(&self) -> PyResult<PyObject> {
        let tabular = tabularize(&self.history)?;
        Python::with_gil(|py| pythonize(py, &tabular))
            .map_err(Into::into)
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
