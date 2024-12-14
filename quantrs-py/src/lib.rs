use numpy::{PyArray1, ToPyArray};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use ::quantrs::trend;

#[pyfunction]
fn sma(py: Python, data: Vec<f64>, period: i64) -> PyResult<Py<PyArray1<f64>>> {
    if period <= 0 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Period must be greater than 0",
        ));
    }
    let period = period as usize;
    let result = trend::sma(&data, period);
    Ok(result.to_pyarray(py).to_owned())
}

#[pymodule]
fn quantrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sma, m)?)?;
    Ok(())
}
