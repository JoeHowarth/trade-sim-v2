#![allow(unused_imports)]

use pyo3::prelude::*;
use simulation::{history::History, prelude::*};

// #[pyfunction]
// fn price(
//     // py: Python<'_>,
//     obj: &PyObject,
// ) -> PyResult<f64> {
//     let (consumption, supply, production, pricer): (
//         f64,
//         f64,
//         f64,
//         (f64, f64, f64),
//     ) = obj.extract(py);
//     todo!()
// }

#[pyfunction]
fn foo(x: u32) -> PyResult<u32> {
    Ok(x * 2 + 13)
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn simrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(foo, m)?)?;
    // m.add_function(wrap_pyfunction!(price, m)?)?;
    Ok(())
}
