use ftp_core::FtpResult; // Remplacez par le nom réel de votre crate
use ndarray::array;

#[test]
fn test_ftp_result_compute_stock_method() {
    // Arrange
    let v_outstanding = array![[1000.0], [1200.0], [1350.0]];
    let m_profile = array![
        [1.00, 0.50, 0.20, 0.05],
        [1.00, 0.50, 0.20, 0.05],
        [1.00, 0.50, 0.20, 0.05]
    ];
    let m_taux = array![
        [0.01300, 0.01400, 0.01600],
        [0.01360, 0.01460, 0.01660],
        [0.01430, 0.01530, 0.01730]
    ];

    // Act
    let mut ftp_res = FtpResult::new(v_outstanding, m_profile, m_taux);
    ftp_res.compute("stock".to_string());

    // Assert
    assert!(ftp_res.stock_amort.is_some());
    assert!(ftp_res.stock_instal.is_some());
    assert!(ftp_res.varstock_amort.is_some());
    assert!(ftp_res.ftp_rate.is_some());
    assert!(ftp_res.market_rate.is_some());

    let stock_amort = ftp_res.stock_amort.unwrap();
    assert_eq!(stock_amort.dim(), (3, 4));
}

#[test]
fn test_ftp_result_compute_flux_method() {
    // Arrange
    let v_outstanding = array![[800.0], [900.0]];
    let m_profile = array![[1.00, 0.60, 0.30], [1.00, 0.60, 0.30]];
    let m_taux = array![[0.01200, 0.01300], [0.01250, 0.01350]];

    // Act
    let mut ftp_res = FtpResult::new(v_outstanding, m_profile, m_taux);
    ftp_res.compute("flux".to_string());

    // Assert
    assert!(ftp_res.varstock_amort.is_some());
    assert!(ftp_res.ftp_int.is_some());

    let varstock_amort = ftp_res.varstock_amort.unwrap();
    assert_eq!(varstock_amort.dim(), (2, 3));
}

#[test]
#[should_panic(expected = "Toutes les matrices d'inputs doivent avoir le meme nombre de ligne")]
fn test_ftp_result_invalid_dimensions() {
    let v_outstanding = array![[1000.0], [1200.0]]; // 2 rows
    let m_profile = array![[1.00, 0.50]]; // 1 row
    let m_taux = array![[0.01300]]; // 1 row

    let mut ftp_res = FtpResult::new(v_outstanding, m_profile, m_taux);
    ftp_res.compute("stock".to_string());
}

#[test]
fn test_ftp_result_new_creation() {
    // Arrange
    let v_outstanding = array![[1000.0]];
    let m_profile = array![[1.00, 0.50]];
    let m_taux = array![[0.01300]];

    // Act
    let ftp_res = FtpResult::new(v_outstanding.clone(), m_profile.clone(), m_taux.clone());

    // Assert
    assert_eq!(ftp_res.input_outstanding.dim(), (1, 1));
    assert_eq!(ftp_res.input_profiles.dim(), (1, 2));
    assert_eq!(ftp_res.input_rate.dim(), (1, 1));
    assert!(ftp_res.stock_amort.is_none());
    assert!(ftp_res.ftp_rate.is_none());
}
