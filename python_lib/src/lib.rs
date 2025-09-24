use ftp_core::common::tmp_calculer::cus_addition;
use pyo3::prelude::*;

#[pyfunction]
fn py_calculer(a: f64, b: f64) -> PyResult<f64> {
    Ok(cus_addition(a, b))
}

#[pymodule]
fn ftp_core_python(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_calculer, m)?)?;
    println!("Hello, world!");
    println!("Hello, world!2");
    Ok(())
}
