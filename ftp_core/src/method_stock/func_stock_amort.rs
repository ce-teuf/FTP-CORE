use crate::holding_struct::FtpResult;

pub fn func_stock_amort(ftp_result: &mut FtpResult, rownum: usize, colnum: usize) {
    // Access the input_outstanding and input_profiles arrays
    let v = &ftp_result.input_outstanding;
    let m = &ftp_result.input_profiles;

    // println!("ok");
    // Check if stock_amort is Some and mutate it
    if let Some(stock_amort) = &mut ftp_result.stock_amort {
        stock_amort[[rownum, colnum]] = v[[rownum, 0]] * m[[rownum, colnum]];
    } else {
        // Handle the case where stock_amort is None, if necessary
        eprintln!("stock_amort is None, cannot update value.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::holding_struct::FtpResult;
    use ndarray::{array, Array2};

    #[test]
    fn test_func_stock_amort_calculation() {
        let mut ftp_result = FtpResult::new(
            array![[1000.0]],
            array![[1.0, 0.5, 0.2]],
            array![[0.01, 0.02]]
        );

        let (nrows, ncols) = ftp_result.input_profiles.dim();
        ftp_result.stock_amort = Some(Array2::<f64>::zeros((nrows, ncols)));

        func_stock_amort(&mut ftp_result, 0, 0);

        let stock_amort = ftp_result.stock_amort.unwrap();
        assert_eq!(stock_amort[[0, 0]], 1000.0); // outstanding[0,0] * profile[0,0]
    }
}