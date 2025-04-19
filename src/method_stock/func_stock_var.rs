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