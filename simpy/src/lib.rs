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
#[derive(From, Into)]
pub struct HistoryPy {
    pub history: History,
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
            self.history.step()?;
        }
        Ok(())
    }

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
        "hi".into()
    }

/// A Python module implemented in Rust.
#[pymodule]
fn simrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(bar, m)?)?;
    m.add_class::<HistoryPy>()?;
    Ok(())
}
