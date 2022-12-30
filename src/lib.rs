use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod tsp_solver;
use tsp_solver::solve;

#[pyfunction]
fn solve_tsp(cost_matrix: Vec<Vec<f64>>) -> PyResult<f64> {
    let answer = solve(&cost_matrix);
    Ok(answer)
}

// ======================CREATING MODULE FOR PYTHON==================================================
/// This module is a python module implemented in Rust.
#[pymodule]
fn travel_salesman_problem(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(solve_tsp))?;
    Ok(())
}
