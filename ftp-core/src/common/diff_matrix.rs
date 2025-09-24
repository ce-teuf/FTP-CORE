pub fn diff_matrix(mut ftp_result: FtpResult) {
    
    let (nrows, ncols) = ftp_result.dim();
    let mut res = Array2::<f64>::zeros((nrows, ncols));

    for i in 0..matrix.nrows() {
        for j in 1..matrix.ncols() {
            res[[i, j]] = matrix[[i, j-1]] - matrix[[i, j]];
        }
    }

    res
}