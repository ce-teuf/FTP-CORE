use pyo3::prelude::*;
use ftp_core::common::tmp_calculer::cus_addition;

#[pyfunction]
fn py_calculer(a: f64, b: f64) -> PyResult<f64> {
    Ok(cus_addition(a, b))
}

// #[pymodule]
// fn votre_projet_python(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(py_calculer, m)?)?;
//     Ok(())
// }
