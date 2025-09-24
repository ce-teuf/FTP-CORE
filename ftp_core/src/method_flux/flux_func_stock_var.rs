use crate::holding_struct::FtpResult;

fn max_zero(a: &f64) -> f64 {
    if *a < 0.0 { 0.0 } else { *a }
}
pub fn flux_func_stock_var(ftp_result: &mut FtpResult,
                         rownum: usize, 
                         colnum: usize) {


    let v_oust = &ftp_result.input_outstanding;


    let m_profile = &ftp_result.input_profiles;

    // println!("ok");
    // Check if stock_amort is Some and mutate it
    if let Some(varstock_amort) = &mut ftp_result.varstock_amort {
        if rownum == 0 {
            varstock_amort[[rownum, colnum]] = m_profile[[rownum, colnum]] * v_oust[[rownum, 0]];
        }
        else {
            if colnum == 0 {
                let mut front_amt: f64 = 0.0;
                for i in 1..(rownum+1) {
                    front_amt = front_amt + varstock_amort[[rownum-i, i]];
                }
                front_amt = v_oust[[rownum, 0]] - front_amt;
                varstock_amort[[rownum, colnum]] = max_zero(&front_amt);

            }
            else {
                varstock_amort[[rownum, colnum]] = varstock_amort[[rownum, 0]] * m_profile[[rownum, colnum]] ;
            }
        }

    } else {
        // Handle the case where stock_amort is None, if necessary
        eprintln!("varstock_amort is None, cannot update value.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::holding_struct::FtpResult;
    use ndarray::{array, Array2};

    #[test]
    fn test_flux_func_stock_var_first_row() {
        let mut ftp_result = FtpResult::new(
            array![[1000.0]],
            array![[1.0, 0.5, 0.2]],
            array![[0.01, 0.02]]
        );

        let (nrows, ncols) = ftp_result.input_profiles.dim();
        ftp_result.varstock_amort = Some(Array2::<f64>::zeros((nrows, ncols)));

        flux_func_stock_var(&mut ftp_result, 0, 0);

        let varstock_amort = ftp_result.varstock_amort.unwrap();
        assert_eq!(varstock_amort[[0, 0]], 1000.0); // profile[0,0] * outstanding[0,0]
    }

    #[test]
    fn test_max_zero_function() {
        assert_eq!(max_zero(&5.0), 5.0);
        assert_eq!(max_zero(&-5.0), 0.0);
        assert_eq!(max_zero(&0.0), 0.0);
    }
}