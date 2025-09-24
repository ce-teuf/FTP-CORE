use crate::holding_struct::FtpResult;

pub fn func_ftp_int(ftp_result: &mut FtpResult,
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


    if let Some(ftp_int) = &mut ftp_result.ftp_int {

        if rownum == 0 {
            //println!("row0 - rownum = {} ; colnum = {}; ncols = {}", rownum, colnum, ncols);
            let mut num = 0.0;

            for k in colnum..ncols-1 {
                num += m_varstock_instal[[0, k+1]] * m_input_rate[[0, k]];
            }

            ftp_int[[rownum, colnum]] = num/12.0;

        }
        else { // rownum > 0

                let mut num1 = 0.0;
                let mut num2 = 0.0;

                for k in colnum..ncols-1 {

                    num1 += m_varstock_instal[[rownum, k+1]] * m_input_rate[[rownum, k]];
                    if k > colnum {
                        num2 += m_stock_instal[[rownum-1, k+1]] * m_market_rate[[rownum-1, k+1]];
                    }
                    
                    
                }

            ftp_int[[rownum, colnum]] = (num1+num2) / 12.0;

            }

    // ftp_rate[[rownum, colnum]] = m_input_rate[[rownum, colnum]];
    } else {
        // Handle the case where varstock_amort is None, if necessary
        eprintln!("ftp_rate is None, cannot update value.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::holding_struct::FtpResult;
    use ndarray::{array, Array2};

    fn create_test_ftp_result() -> FtpResult {
        let outstanding = array![[1000.0]];
        let profiles = array![[1.0, 0.5, 0.2]];
        let rates = array![[0.01, 0.02]];

        let mut ftp_result = FtpResult::new(outstanding, profiles, rates);

        let (nrows, ncols) = ftp_result.input_profiles.dim();
        ftp_result.varstock_instal = Some(Array2::<f64>::ones((nrows, ncols)));
        ftp_result.stock_instal = Some(Array2::<f64>::ones((nrows, ncols)));
        ftp_result.market_rate = Some(Array2::<f64>::ones((nrows, ncols)));
        ftp_result.ftp_int = Some(Array2::<f64>::zeros((nrows, ncols)));

        ftp_result
    }

    #[test]
    fn test_func_ftp_int_calculation() {
        let mut ftp_result = create_test_ftp_result();
        let ncols = ftp_result.input_profiles.dim().1;

        func_ftp_int(&mut ftp_result, 0, 0, ncols);

        assert!(ftp_result.ftp_int.is_some());
        let ftp_int = ftp_result.ftp_int.unwrap();
        assert!(ftp_int[[0, 0]].is_finite());
    }

    #[test]
    fn test_func_ftp_int_division_by_zero_handling() {
        let mut ftp_result = create_test_ftp_result();
        let ncols = ftp_result.input_profiles.dim().1;

        // Configurer pour provoquer une division par zéro
        if let Some(ref mut varstock_instal) = ftp_result.varstock_instal {
            varstock_instal.fill(0.0);
        }

        func_ftp_int(&mut ftp_result, 0, 0, ncols);

        // Vérifier que la fonction ne panique pas
        assert!(ftp_result.ftp_int.is_some());
    }
}