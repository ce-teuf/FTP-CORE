use std::sync::RwLock;

use crate::holding_struct::FtpResult;

pub fn func_ftp_rate(ftp_result: &mut FtpResult, 
                         rownum: usize, 
                         colnum: usize,
                         ncols: usize) {

    let m_input_rate = &ftp_result.input_rate;

    let m_stock_instal = match &ftp_result.stock_instal {
        Some(m_stock_instal) => m_stock_instal,
        None => {
            eprintln!("m_stock_instal is None, cannot read value.");
            return;
        }
    };

    let m_varstock_instal = match &ftp_result.varstock_instal {
        Some(m_varstock_instal) => m_varstock_instal,
        None => {
            eprintln!("m_varstock_instal is None, cannot read value.");
            return;
        }
    };

    let m_market_rate = match &ftp_result.market_rate {
        Some(m_market_rate) => m_market_rate,
        None => {
            eprintln!("market_rate is None, cannot read value.");
            return;
        }
    };


    if let Some(ftp_rate) = &mut ftp_result.ftp_rate {

        if colnum < ncols - 2 {
            if rownum == 0 {
                //println!("row0 - rownum = {} ; colnum = {}; ncols = {}", rownum, colnum, ncols);
                let mut num = 0.0;
                let mut denum = 0.0;

                for k in colnum..ncols-1 {
                    
                    //println!("m_varstock_instal: {:?}", m_varstock_instal);
                    //println!("Accessing m_varstock_instal[[0, {}]]", k+1);
                    //println!("Value: {}", m_varstock_instal[[0, k+1]]);
                    num = num + (m_varstock_instal[[0, k+1]] * m_input_rate[[0, k]]);
                    denum = denum + m_varstock_instal[[0, k+1]];
                }
                //println!("num = {}", num);
                //println!("denum = {}", denum);
                if denum != 0.0 {
                    ftp_rate[[rownum, colnum]] = num/denum;
                }
                else {
                    ftp_rate[[rownum, colnum]] = 0.0;
                }
                
            }
            else { // rownum > 0
                //println!("rownum = {} ; colnum = {}; ncols = {}", rownum, colnum, ncols);
                let mut num1 = 0.0;
                let mut num2 = 0.0;

                let mut denum1 = 0.0;
                let mut denum2 = 0.0;

                for k in colnum..ncols-1 {
                    println!("ftp test = {}", m_input_rate[[rownum, k]]);
                    num1 += m_varstock_instal[[rownum, k+1]] * m_input_rate[[rownum, k]];
                    denum1 += m_varstock_instal[[0, k]];
                    if k > colnum {
                        num2 += m_stock_instal[[rownum-1, k+1]] * m_market_rate[[rownum-1, k]];
                        denum2 += m_stock_instal[[0, k]];
                    }
                    
                    
                }
                if denum1 != 0.0 {
                    ftp_rate[[rownum, colnum]] = num1/denum1;
                }
                else {
                    ftp_rate[[rownum, colnum]] = 0.0;
                }
                if denum2 != 0.0 {
                    ftp_rate[[rownum, colnum]] = num2/denum2;
                }
                else {
                    ftp_rate[[rownum, colnum]] = 0.0;
                }
            }
        } else if colnum == ncols - 2  { // colnum == ncols - 1
            //println!("ex rownum = {} ; colnum = {}; ncols = {}", rownum, colnum, ncols);
            ftp_rate[[rownum, colnum]] = m_input_rate[[rownum, colnum]];
        }
    } else {
        // Handle the case where varstock_amort is None, if necessary
        eprintln!("ftp_rate is None, cannot update value.");
    }
}