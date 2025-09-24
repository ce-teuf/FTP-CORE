use ftp_core::common::tmp_calculer::cus_addition;

#[no_mangle]
pub extern "C" fn excel_calculer(a: f64, b: f64) -> f64 {
    cus_addition(a, b) // Appel au noyau
}
