use crate::holding_struct::FtpResult;

pub fn func_stock_var_instal(ftp_result: &mut FtpResult, rownum: usize, colnum: usize) {
    let m = match &ftp_result.varstock_amort {
        Some(varstock_amort) => varstock_amort,
        None => {
            eprintln!("varstock_amort is None, cannot read value.");
            return;
        }
    };
    // println!("ok");
    // Check if varstock_amort is Some and mutate it
    if let Some(varstock_instal) = &mut ftp_result.varstock_instal {
        if colnum > 0 {
            varstock_instal[[rownum, colnum]] = m[[rownum, colnum - 1]] - m[[rownum, colnum]];
        }
    } else {
        // Handle the case where varstock_amort is None, if necessary
        eprintln!("varstock_instal is None, cannot update value.");
    }
}
