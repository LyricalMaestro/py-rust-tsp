use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn cumulate(n: Vec<i32>) -> PyResult<Vec<i32>> {
    let mut cum_n: Vec<i32> = vec![0;n.len()];
    let mut c: i32 = 0;
    for i in 0..n.len() {
        c += n[i];
        cum_n[i] = c;
    }
    Ok(cum_n)
}

// ======================CREATING MODULE FOR PYTHON==================================================
/// This module is a python module implemented in Rust.
#[pymodule]
fn test_library(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(cumulate))?;

    Ok(())
}
