use crate::holding_struct::FtpResult;

pub fn func_stock_var(ftp_result: &mut FtpResult, 
                         rownum: usize, 
                         colnum: usize,
                         ncols: usize) {


    let m = match &ftp_result.stock_amort {
        Some(stock_amort) => stock_amort,
        None => {
            eprintln!("stock_amort is None, cannot read value.");
            return;
        }
    };
    // println!("ok");
    // Check if stock_amort is Some and mutate it
    if let Some(varstock_amort) = &mut ftp_result.varstock_amort {
        if rownum == 0 {
            varstock_amort[[rownum, colnum]] = m[[rownum, colnum]];
        }
        else {
            if colnum == ncols - 1 {
                varstock_amort[[rownum, colnum]] = m[[rownum, colnum]];
            }
            else {
                varstock_amort[[rownum, colnum]] = m[[rownum, colnum]] - m[[rownum-1, colnum+1]];
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
    fn test_func_stock_var_first_row() {
        let mut ftp_result = FtpResult::new(
            array![[1000.0]],
            array![[1.0, 0.5, 0.2]],
            array![[0.01, 0.02]]
        );

        let (nrows, ncols) = ftp_result.input_profiles.dim();

        // Initialiser stock_amort
        let mut stock_amort = Array2::<f64>::zeros((nrows, ncols));
        stock_amort[[0, 0]] = 1000.0;
        stock_amort[[0, 1]] = 500.0;
        stock_amort[[0, 2]] = 200.0;

        ftp_result.stock_amort = Some(stock_amort);
        ftp_result.varstock_amort = Some(Array2::<f64>::zeros((nrows, ncols)));

        func_stock_var(&mut ftp_result, 0, 1, ncols);

        let varstock_amort = ftp_result.varstock_amort.unwrap();
        assert_eq!(varstock_amort[[0, 1]], 500.0); // Égal à stock_amort[0,1] pour la première ligne
    }
}