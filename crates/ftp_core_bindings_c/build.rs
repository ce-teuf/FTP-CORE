extern crate cbindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Chemin de sortie pour le header C
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = PathBuf::from(&crate_dir)
        .join("..")
        .join("excel")
        .join("Interop");

    // Crée le dossier Interop s'il n'existe pas
    std::fs::create_dir_all(&out_dir).unwrap();

    // Génère le header C
    cbindgen::generate(crate_dir)
        .unwrap()
        .write_to_file(out_dir.join("bindings.h"));

    // Imprime le chemin du header pour débogage
    println!(
        "cargo:warning=Header C généré dans {:?}",
        out_dir.join("bindings.h")
    );
}
