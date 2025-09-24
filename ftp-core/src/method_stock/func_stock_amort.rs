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