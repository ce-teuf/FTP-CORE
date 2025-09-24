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

        if rownum == 0 {

            let mut num = 0.0;
            let mut denum = 0.0;

            for k in colnum..ncols-1 {
                num = num + (m_varstock_instal[[0, k+1]] * m_input_rate[[0, k]]);
                denum = denum + m_varstock_instal[[0, k+1]];
            }

            if denum != 0.0 {
                ftp_rate[[rownum, colnum]] = num/denum;
            }
            else {
                ftp_rate[[rownum, colnum]] = 0.0;
            }

        }
        else { // rownum > 0
                let mut num1 = 0.0;
                let mut num2 = 0.0;

                let mut denum1 = 0.0;
                let mut denum2 = 0.0;

                for k in colnum..ncols-1 {

                    num1 += m_varstock_instal[[rownum, k+1]] * m_input_rate[[rownum, k]];
                    denum1 += m_varstock_instal[[rownum, k+1]];
                    if k > colnum {
                        num2 += m_stock_instal[[rownum-1, k+1]] * m_market_rate[[rownum-1, k+1]];
                        denum2 += m_stock_instal[[rownum-1, k+1]];
                    }
                    
                    
                }

                if denum1 + denum2 != 0.0 {
                    ftp_rate[[rownum, colnum]] = (num1+num2)/(denum1+denum2);
                }
                else {
                    ftp_rate[[rownum, colnum]] = 0.0;
                }
            }

    } else {
        eprintln!("ftp_rate is None, cannot update value.");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::holding_struct::FtpResult;
    use ndarray::{array, Array2};

    fn create_test_ftp_result() -> FtpResult {
        let outstanding = array![[1000.0], [1200.0]];
        let profiles = array![[1.0, 0.5, 0.2], [1.0, 0.5, 0.2]];
        let rates = array![[0.01, 0.02], [0.015, 0.025]];

        let mut ftp_result = FtpResult::new(outstanding, profiles, rates);

        // Initialiser les matrices optionnelles nécessaires
        let (nrows, ncols) = ftp_result.input_profiles.dim();
        ftp_result.varstock_instal = Some(Array2::<f64>::ones((nrows, ncols)));
        ftp_result.stock_instal = Some(Array2::<f64>::ones((nrows, ncols)));
        ftp_result.market_rate = Some(Array2::<f64>::ones((nrows, ncols)));
        ftp_result.ftp_rate = Some(Array2::<f64>::zeros((nrows, ncols)));

        ftp_result
    }

    #[test]
    fn test_func_ftp_rate_first_row() {
        let mut ftp_result = create_test_ftp_result();
        let ncols = ftp_result.input_profiles.dim().1;

        func_ftp_rate(&mut ftp_result, 0, 0, ncols);

        assert!(ftp_result.ftp_rate.is_some());
        let ftp_rate = ftp_result.ftp_rate.unwrap();
        // Vérifier que la valeur a été calculée
        assert_ne!(ftp_rate[[0, 0]], 0.0);
    }

    #[test]
    fn test_func_ftp_rate_handles_none_values() {
        let mut ftp_result = FtpResult::new(
            array![[1000.0]],
            array![[1.0, 0.5]],
            array![[0.01]]
        );

        // Ne pas initialiser les matrices optionnelles
        func_ftp_rate(&mut ftp_result, 0, 0, 2);

        // Devrait gérer gracieusement les valeurs None
        // (Le test passe si aucune panique ne se produit)
    }
}

