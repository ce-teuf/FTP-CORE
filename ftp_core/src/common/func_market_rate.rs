use crate::holding_struct::FtpResult;

pub fn func_market_rate(ftp_result: &mut FtpResult, rownum: usize, colnum: usize, ncols: usize) {
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
            market_rate[[rownum, colnum]] = m_input_rate[[rownum, colnum - 1]];
        } else {
            // colnum < ncols - 1
            let a = m_ftp_rate[[rownum, colnum - 1]];
            let mut b = 0.0;
            let mut c = 0.0;
            let d = m_stock_instal[[rownum, colnum]];

            for k in colnum..ncols {
                b += m_stock_instal[[rownum, k]];
            }

            for k in colnum + 1..ncols {
                c += m_stock_instal[[rownum, k]] * market_rate[[rownum, k]];
            }

            if d != 0.0 {
                market_rate[[rownum, colnum]] = ((a * b) - c) / d;
            } else {
                market_rate[[rownum, colnum]] = 0.0;
            }
        }
    } else {
        // Handle the case where varstock_amort is None, if necessary
        eprintln!("market_rate is None, cannot update value.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::holding_struct::FtpResult;
    use ndarray::{array, Array2};

    #[test]
    fn test_func_market_rate_last_column() {
        let mut ftp_result = FtpResult::new(
            array![[1000.0]],
            array![[1.0, 0.5, 0.2]],
            array![[0.01, 0.02]],
        );

        let (nrows, ncols) = ftp_result.input_profiles.dim();
        ftp_result.stock_instal = Some(Array2::<f64>::ones((nrows, ncols)));
        ftp_result.ftp_rate = Some(Array2::<f64>::ones((nrows, ncols)));
        ftp_result.market_rate = Some(Array2::<f64>::zeros((nrows, ncols)));

        func_market_rate(&mut ftp_result, 0, ncols - 1, ncols);

        assert!(ftp_result.market_rate.is_some());
        let market_rate = ftp_result.market_rate.unwrap();
        assert_eq!(market_rate[[0, ncols - 1]], 0.02); // Devrait être input_rate[0, ncols-2]
    }
}
