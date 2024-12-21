use numpy::{self as np, ToPyArray};
use pyo3::prelude::*;
use pyo3::{exceptions::PyValueError, wrap_pyfunction};

use ::quantrs::{error::QuantrsError, trend};

fn to_pyerr(e: QuantrsError) -> PyErr {
    match e {
        QuantrsError::InvalidPeriod => {
            PyErr::new::<PyValueError, _>("Period must be greater than 0")
        }
        QuantrsError::EmptyData => PyErr::new::<PyValueError, _>("Data cannot be empty"),
    }
}

#[pyfunction]
fn sma(py: Python, data: Vec<f64>, period: i64) -> PyResult<Py<np::PyArray1<f64>>> {
    match trend::sma(&data, period as usize) {
        Ok(result) => Ok(result.to_pyarray(py).to_owned()),
        Err(e) => Err(to_pyerr(e)),
    }
}

#[pymodule]
fn quantrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sma, m)?)?;
    Ok(())
}
