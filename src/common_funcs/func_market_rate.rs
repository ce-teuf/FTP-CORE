use crate::holding_struct::FtpResult;

pub fn func_market_rate(ftp_result: &mut FtpResult, 
                         rownum: usize, 
                         colnum: usize,
                         ncols: usize) {

    let m_stock_instal = match &ftp_result.stock_instal {
        Some(m_stock_instal) => m_stock_instal,
        None => {
            eprintln!("m_stock_instal is None, cannot read value.");
            return;
        }
    };

    let m_ftp_rate = match &ftp_result.ftp_rate {
        Some(m_ftp_rate) => m_ftp_rate,
        None => {
            eprintln!("m_varstock_instal is None, cannot read value.");
            return;
        }
    };

    let m_input_rate = &ftp_result.input_rate;

    if let Some(market_rate) = &mut ftp_result.market_rate {
        if colnum == ncols - 1 {
            market_rate[[rownum, colnum]] = m_input_rate[[rownum, colnum-1]];

        } else { // colnum < ncols - 1
            let a = m_ftp_rate[[rownum, colnum-1]];
            let mut b = 0.0;
            let mut c = 0.0;
            let d = m_stock_instal[[rownum, colnum]];
            
            for k in colnum..ncols {
                b = b + m_stock_instal[[rownum, k]];
            }

            for k in colnum+1..ncols {
                c = c + ( m_stock_instal[[rownum, k]] * market_rate[[rownum, k]] );
            }

            if d != 0.0 {
                market_rate[[rownum, colnum]] = ((a * b) - c) / d;
            }
            else {
                market_rate[[rownum, colnum]] = 0.0;
            }
            
        }
    } else {
        // Handle the case where varstock_amort is None, if necessary
        eprintln!("market_rate is None, cannot update value.");
    }
}