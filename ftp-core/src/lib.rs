use ndarray::array;
// use pyo3::prelude::*;
mod utils;

mod holding_struct;

mod method_stock;
mod method_flux;
pub mod common;



pub use crate::holding_struct::FtpResult;
//pub use crate::holding_struct::PyFtpResult;


use ndarray::Array2;
//use numpy::{PyReadonlyArray2, PyArray2};



pub fn mainx() {

    let v_outstanding = array![
        [1000.0],
        [1200.0],
        [1350.0],
        [1250.0],
        [1380.0]
    ];

    let m_profile = array![[1.00,	0.50,	0.20,	0.05, 0.00],
                                                            [1.00,	0.50,	0.20,	0.05, 0.00],
                                                            [1.00,	0.50,	0.20,	0.05, 0.00],
                                                            [1.00,	0.50,	0.20,	0.05, 0.00],
                                                            [1.00,	0.50,	0.20,	0.05, 0.00]];
    let m_taux = array!
    [[0.01300, 0.01400, 0.01600, 0.01700],
    [0.01360, 0.01460, 0.01660, 0.01770],
    [0.01430, 0.01530, 0.01730, 0.01840],
    [0.01470, 0.01570, 0.01780, 0.01890],
    [0.01500, 0.01600, 0.01820, 0.01920]];

    let v_outstanding2 = v_outstanding.clone();
    let m_profile2 = m_profile.clone();
    let m_taux2 = m_taux.clone();

    let mut ftp_res1 = FtpResult::new(v_outstanding, m_profile, m_taux);
    ftp_res1.compute("stock".to_string());
    
    //println!("{:.6}\n", ftp_res1.ftp_rate.unwrap());
    //println!("{:.6}\n", ftp_res1.market_rate.unwrap());
    //println!("{:.6}", ftp_res1.ftp_int.unwrap());
    //println!("test");

    let mut ftp_res2 = FtpResult::new(v_outstanding2, m_profile2, m_taux2);
    ftp_res2.compute("flux".to_string());

    println!("{:.6}\n", ftp_res2.ftp_rate.unwrap());
    println!("{:.6}\n", ftp_res2.market_rate.unwrap());
    println!("{:.6}\n", ftp_res2.varstock_amort.unwrap());
    println!("{:.6}", ftp_res2.stock_amort.unwrap());
    println!("test");
}
