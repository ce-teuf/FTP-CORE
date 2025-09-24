use crate::holding_struct::FtpResult;

pub fn func_stock_instal(ftp_result: &mut FtpResult, rownum: usize, colnum: usize) {
    let m = match &ftp_result.stock_amort {
        Some(stock_amort) => stock_amort,
        None => {
            eprintln!("stock_amort is None, cannot read value.");
            return;
        }
    };
    // println!("ok");
    // Check if stock_amort is Some and mutate it
    if let Some(stock_instal) = &mut ftp_result.stock_instal {
        if colnum > 0 {
            stock_instal[[rownum, colnum]] = m[[rownum, colnum - 1]] - m[[rownum, colnum]];
        }
    } else {
        // Handle the case where stock_amort is None, if necessary
        eprintln!("stock_instal is None, cannot update value.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::holding_struct::FtpResult;
    use ndarray::{array, Array2};

    #[test]
    fn test_func_stock_instal_calculation() {
        let mut ftp_result = FtpResult::new(
            array![[1000.0]],
            array![[1.0, 0.5, 0.2]],
            array![[0.01, 0.02]],
        );

        let (nrows, ncols) = ftp_result.input_profiles.dim();

        // Créer une matrice stock_amort avec des valeurs connues
        let mut stock_amort = Array2::<f64>::zeros((nrows, ncols));
        stock_amort[[0, 0]] = 1000.0;
        stock_amort[[0, 1]] = 500.0;
        stock_amort[[0, 2]] = 200.0;

        ftp_result.stock_amort = Some(stock_amort);
        ftp_result.stock_instal = Some(Array2::<f64>::zeros((nrows, ncols)));

        func_stock_instal(&mut ftp_result, 0, 1);

        assert!(ftp_result.stock_instal.is_some());
        let stock_instal = ftp_result.stock_instal.unwrap();
        assert_eq!(stock_instal[[0, 1]], 500.0); // 1000 - 500
    }

    #[test]
    fn test_func_stock_instal_first_column() {
        let mut ftp_result = FtpResult::new(
            array![[1000.0]],
            array![[1.0, 0.5, 0.2]],
            array![[0.01, 0.02]],
        );

        let (nrows, ncols) = ftp_result.input_profiles.dim();
        ftp_result.stock_amort = Some(Array2::<f64>::ones((nrows, ncols)));
        ftp_result.stock_instal = Some(Array2::<f64>::zeros((nrows, ncols)));

        func_stock_instal(&mut ftp_result, 0, 0);

        // Pour colnum = 0, la valeur ne devrait pas changer
        let stock_instal = ftp_result.stock_instal.unwrap();
        assert_eq!(stock_instal[[0, 0]], 0.0);
    }
}
