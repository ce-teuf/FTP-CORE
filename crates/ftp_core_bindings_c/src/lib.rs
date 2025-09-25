// Import de la crate core (à décommenter si tu l'as déjà)
// use core::fonction_rust;

// Fonction Rust exposée en C
#[no_mangle]  // Désactive le "name mangling" pour que le nom reste intact
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    // Exemple minimal : addition de deux entiers
    a + b

    // Si tu as une crate `core`, tu peux appeler ses fonctions ici :
    // core::fonction_rust(a, b)
}

// Fonction pour tester les erreurs (optionnel)
#[no_mangle]
pub extern "C" fn safe_divide(numerator: i32, denominator: i32) -> i32 {
    if denominator == 0 {
        -1  // Code d'erreur (à adapter selon tes besoins)
    } else {
        numerator / denominator
    }
}
