# FTP Core

Bibliothèque Rust principale pour les calculs FTP.

## Structure des données

### FtpResult
Structure principale contenant toutes les matrices de calcul.

```rust
use ftp_core::FtpResult;
use ndarray::array;

let mut result = FtpResult::new(
    array![[1000.0]], 
    array![[1.0, 0.5, 0.2]],
    array![[0.01, 0.02]]
);
```

## Méthodes de calcul

Deux méthodes sont supportées :
- **Stock** : Méthode traditionnelle
- **Flux** : Méthode basée sur les flux






