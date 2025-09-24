use ndarray::s;
use crate::holding_struct::FtpResult;
use crate::utils::extract_anti_diagonal_rect2;

pub fn flux_func_stock_amort(ftp_result: &mut FtpResult, rownum: usize, colnum: usize) {

    let m_prod = match &ftp_result.varstock_amort {
        Some(m_prod) => m_prod,
        None => {
            eprintln!("stock_amort is None, cannot read value.");
            return;
        }
    };

    let (_, ncols) = m_prod.dim();
    // println!("ok");
    // Check if stock_amort is Some and mutate it
    if let Some(stock_amort) = &mut ftp_result.stock_amort {
        if rownum == 0 {
            stock_amort[[rownum, colnum]] = m_prod[[rownum, colnum]];
        }
        else {
            let aa = m_prod.slice(s![0..rownum+1, colnum..ncols]);
            let bb = extract_anti_diagonal_rect2(&aa);

            stock_amort[[rownum, colnum]] = bb.iter().sum::<f64>();
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
    fn test_flux_func_stock_amort_first_row() {
        let mut ftp_result = FtpResult::new(
            array![[1000.0]],
            array![[1.0, 0.5, 0.2]],
            array![[0.01, 0.02]]
        );

        let (nrows, ncols) = ftp_result.input_profiles.dim();
        ftp_result.varstock_amort = Some(Array2::<f64>::ones((nrows, ncols)));
        ftp_result.stock_amort = Some(Array2::<f64>::zeros((nrows, ncols)));

        flux_func_stock_amort(&mut ftp_result, 0, 0);

        let stock_amort = ftp_result.stock_amort.unwrap();
        assert_eq!(stock_amort[[0, 0]], 1.0); // Devrait être égal à varstock_amort[0,0]
    }
}