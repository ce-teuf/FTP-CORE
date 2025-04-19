use crate::holding_struct::FtpResult;

pub fn func_stock_instal(ftp_result: &mut FtpResult, 
                         rownum: usize, 
                         colnum: usize) {


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
            stock_instal[[rownum, colnum]] = m[[rownum, colnum-1]] - m[[rownum, colnum]];
        }

    } else {
        // Handle the case where stock_amort is None, if necessary
        eprintln!("stock_instal is None, cannot update value.");
    }
}