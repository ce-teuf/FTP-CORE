use ndarray::{Array2, array, s};

pub struct FtpResult {
    pub input_outstanding: Array2<f64>,
    pub input_profiles: Array2<f64>,
    pub input_rate: Array2<f64>,
    pub stock_amort: Option<Array2<f64>>,
    pub stock_instal: Option<Array2<f64>>,
    pub varstock_amort: Option<Array2<f64>>, // or new prod amort
    pub varstock_instal: Option<Array2<f64>>, // or new prod instal
    pub ftp_rate: Option<Array2<f64>>,
    pub ftp_int: Option<Array2<f64>>,
    pub market_rate: Option<Array2<f64>>,
}

use crate::method_stock::{func_ftp_rate::func_ftp_rate,
                          func_ftp_int::func_ftp_int,
                          func_market_rate,
    func_stock_amort::func_stock_amort,
    func_stock_instal::func_stock_instal, 
    func_stock_var::func_stock_var, 
    func_stock_var_instal::func_stock_var_instal,
    func_market_rate::func_market_rate
};

impl FtpResult {

    fn check_dims(&self) {
        let (nrows_outs, ncols_outs) = self.input_outstanding.dim();
        let (nrows_profiles, ncols_profiles) = self.input_profiles.dim();
        let (nrows_rate, ncols_rate) = self.input_rate.dim();

        if (nrows_outs != nrows_profiles) || (nrows_outs != nrows_rate) || (nrows_profiles != nrows_rate) {
            panic!("Toutes les matrices d'inputs doivent avoir le meme nombre de ligne");
        }
        if ncols_outs != 1 {
            panic!("'input outstanding' doit avoir une colonne !");
        }
        if ncols_profiles - 1 != ncols_rate {
            panic!("'input_rate' doit avoir une colonne de moins que 'input_profiles'");
        }
    }

    fn init_arrays(&mut self, nrows: usize, ncols: usize) {
        self.stock_amort = Some(Array2::<f64>::zeros((nrows, ncols)));
        self.stock_instal = Some(Array2::<f64>::zeros((nrows, ncols)));
        self.varstock_amort = Some(Array2::<f64>::zeros((nrows, ncols)));
        self.varstock_instal = Some(Array2::<f64>::zeros((nrows, ncols)));
        self.ftp_rate = Some(Array2::<f64>::zeros((nrows, ncols)));
        self.ftp_int = Some(Array2::<f64>::zeros((nrows, ncols)));
        self.market_rate = Some(Array2::<f64>::zeros((nrows, ncols)));
    }
    
    pub fn new(input_outstanding: Array2<f64>, input_profiles: Array2<f64>, input_rate: Array2<f64>) -> Self {
        Self {
            input_outstanding: input_outstanding,
            input_profiles: input_profiles,
            input_rate: input_rate,
            stock_amort: None,
            stock_instal: None,
            varstock_amort: None,
            varstock_instal: None,
            ftp_rate: None,
            ftp_int: None,
            market_rate: None,
        }
    }

    pub fn compute(&mut self, method: String) {
        // Check dimensions
        self.check_dims();
        
        let (nrows, ncols) = self.input_profiles.dim();

        self.init_arrays(nrows, ncols);

        if method == "stock".to_string() {
            // Implementation for "stock" method
            // stock amort
            for i in 0..nrows {
                for j in 0..ncols {
                    func_stock_amort(self, i, j);
                    func_stock_instal(self, i, j);
                    func_stock_var(self, i, j, ncols);
                    func_stock_var_instal(self, i, j);
                }

                for j in (0..ncols).rev() {
                    if j > 0 {
                        //println!("\n");
                        func_ftp_rate(self, i, j-1, ncols);
                        func_ftp_int(self, i, j-1, ncols);
                        let aa = self.ftp_rate.as_ref().unwrap();
                        //println!("i = {}; j-1 = {} ; ftp = {:.5}",i, j-1 , aa[[i, j-1]]);

                        func_market_rate(self, i, j, ncols);

                        let bb = self.market_rate.as_ref().unwrap();
                        //println!("i = {}; j = {} ; market = {:.5}",i, j , bb[[i, j]]);
                    }
                }


            }

        } else if method == "flux".to_string() {
            // Implementation for "flux" method
            // Implementation for "stock" method
            // stock amort
            for i in 0..nrows {
                for j in 0..ncols {
                    func_stock_amort(self, i, j);
                    func_stock_instal(self, i, j);
                    func_stock_var(self, i, j, ncols);
                    func_stock_var_instal(self, i, j);
                }

                for j in (0..ncols).rev() {
                    if j > 0 {
                        //println!("\n");
                        func_ftp_rate(self, i, j-1, ncols);
                        func_ftp_int(self, i, j-1, ncols);
                        func_market_rate(self, i, j, ncols);

                        let bb = self.market_rate.as_ref().unwrap();
                        //println!("i = {}; j = {} ; market = {:.5}",i, j , bb[[i, j]]);
                    }
                }
            }
        } else {
            // Handle other methods or errors
        }
    }
}
