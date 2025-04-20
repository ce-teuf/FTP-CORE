use ndarray::{Array2, array, s};

mod utils;
use utils::get_shape;
use utils::print_array2;
use utils::extract_anti_diagonal_rect2;

mod core_ftp;
use core_ftp::m_step_1_outstanding_amort;
use core_ftp::m_step_2_outstanding_instalment;
use core_ftp::m_step_3_stockvar;
use core_ftp::m_step_4_stockvar_instalment;
use core_ftp::m_step_5_ftp_rate;
use core_ftp::rebuilt_market_rate;

mod holding_struct;

mod method_stock;

use crate::holding_struct::FtpResult;


fn main() {

    let mut v_outstanding = array![
        [1000.0],
        [1200.0],
        [1350.0],
        [1250.0],
        [1380.0]
    ];

    let mut m_profile = array![[1.00,	0.50,	0.20,	0.05, 0.00],
                                                            [1.00,	0.50,	0.20,	0.05, 0.00],
                                                            [1.00,	0.50,	0.20,	0.05, 0.00],
                                                            [1.00,	0.50,	0.20,	0.05, 0.00],
                                                            [1.00,	0.50,	0.20,	0.05, 0.00]];
    let mut m_taux = array!
    [[0.01300, 0.01400, 0.01600, 0.01700],
    [0.01360, 0.01460, 0.01660, 0.01770],
    [0.01430, 0.01530, 0.01730, 0.01840],
    [0.01470, 0.01570, 0.01780, 0.01890],
    [0.01500, 0.01600, 0.01820, 0.01920]];                                                   

    let mut ftp_res1 = FtpResult::new(v_outstanding, m_profile, m_taux);
    ftp_res1.compute("stock".to_string());
    
    println!("{:.6}\n", ftp_res1.ftp_rate.unwrap());
    println!("{:.6}\n", ftp_res1.market_rate.unwrap());
    println!("{:.6}", ftp_res1.ftp_int.unwrap());
    println!("test");
}
