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