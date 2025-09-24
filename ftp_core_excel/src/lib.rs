use ftp_core::common::tmp_calculer::cus_addition;

/// Fonction exportée pour Excel - Additionne deux nombres
///
/// # Safety
///
/// Cette fonction est safe tant que les inputs sont des nombres valides
///
/// # Arguments
///
/// * `a` - Premier nombre (f64)
/// * `b` - Deuxième nombre (f64)
///
/// # Returns
///
/// La somme des deux nombres (f64)
///
/// # Exemple d'utilisation dans Excel
///
/// ```
/// =excel_calculer(2,5; 3,7)  // Retourne 6,2
/// ```
#[no_mangle]
pub extern "C" fn excel_calculer(a: f64, b: f64) -> f64 {
    cus_addition(a, b) // Appel au noyau
}
