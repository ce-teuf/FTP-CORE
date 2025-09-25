use ftp_core::common::tmp_calculer::cus_addition;
use pyo3::prelude::*;

/// Additionne deux nombres et retourne le résultat
///
/// # Arguments
///
/// * `a` - Premier nombre
/// * `b` - Deuxième nombre
///
/// # Returns
///
/// La somme des deux nombres
///
/// # Exemples
///
/// ```python
/// >>> from ftp_core_bindings_pyo3 import py_calculer
/// >>> py_calculer(2.5, 3.7)
/// 6.2
/// ```
#[pyfunction]
fn py_calculer(a: f64, b: f64) -> PyResult<f64> {
    Ok(cus_addition(a, b))
}

/// Adds two integers.
#[pyfunction]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[pymodule]
fn ftp_core_python(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_calculer, m)?)?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    println!("Hello, world!");
    println!("Hello, world!2");
    Ok(())
}
