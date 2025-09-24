use crate::method_flux::flux_func_stock_amort::flux_func_stock_amort;
use crate::method_flux::flux_func_stock_var::flux_func_stock_var;
use ndarray::Array2;
// use pyo3::prelude::*;
// use pyo3::types::PyType;
// use numpy::PyArray2;

// #[pyclass]
// pub struct PyFtpResult {
//     #[pyo3(get, set)]
//     pub input_outstanding: Py<PyArray2<f64>>,
//     #[pyo3(get, set)]
//     pub input_profiles: Py<PyArray2<f64>>,
//     #[pyo3(get, set)]
//     pub input_rate: Py<PyArray2<f64>>,
//     #[pyo3(get, set)]
//     pub stock_amort: Option<Py<PyArray2<f64>>>,
//     #[pyo3(get, set)]
//     pub stock_instal: Option<Py<PyArray2<f64>>>,
//     #[pyo3(get, set)]
//     pub varstock_amort: Option<Py<PyArray2<f64>>>,
//     #[pyo3(get, set)]
//     pub varstock_instal: Option<Py<PyArray2<f64>>>,
//     #[pyo3(get, set)]
//     pub ftp_rate: Option<Py<PyArray2<f64>>>,
//     #[pyo3(get, set)]
//     pub ftp_int: Option<Py<PyArray2<f64>>>,
//     #[pyo3(get, set)]
//     pub market_rate: Option<Py<PyArray2<f64>>>,
// }

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

//
// impl From<FtpResult> for PyFtpResult {
//     fn from(ftp_result: FtpResult) -> Self {
//         Python::with_gil(|py| {
//             PyFtpResult {
//                 input_outstanding: PyArray2::from_owned_array(py, ftp_result.input_outstanding).into(),
//                 input_profiles: PyArray2::from_owned_array(py, ftp_result.input_profiles).into(),
//                 input_rate: PyArray2::from_owned_array(py, ftp_result.input_rate).into(),
//                 stock_amort: ftp_result.stock_amort.map(|arr| PyArray2::from_owned_array(py, arr).into()),
//                 stock_instal: ftp_result.stock_instal.map(|arr| PyArray2::from_owned_array(py, arr).into()),
//                 varstock_amort: ftp_result.varstock_amort.map(|arr| PyArray2::from_owned_array(py, arr).into()),
//                 varstock_instal: ftp_result.varstock_instal.map(|arr| PyArray2::from_owned_array(py, arr).into()),
//                 ftp_rate: ftp_result.ftp_rate.map(|arr| PyArray2::from_owned_array(py, arr).into()),
//                 ftp_int: ftp_result.ftp_int.map(|arr| PyArray2::from_owned_array(py, arr).into()),
//                 market_rate: ftp_result.market_rate.map(|arr| PyArray2::from_owned_array(py, arr).into()),
//             }
//         })
//     }
// }

// #[pymethods]
// impl PyFtpResult {
//     #[new]
//     fn new(
//         input_outstanding: Py<PyArray2<f64>>,
//         input_profiles: Py<PyArray2<f64>>,
//         input_rate: Py<PyArray2<f64>>,
//     ) -> Self {
//         Self {
//             input_outstanding,
//             input_profiles,
//             input_rate,
//             stock_amort: None,
//             stock_instal: None,
//             varstock_amort: None,
//             varstock_instal: None,
//             ftp_rate: None,
//             ftp_int: None,
//             market_rate: None,
//         }
//     }
//
//     fn __repr__(&self) -> String {
//         Python::with_gil(|py| {
//             format!(
//                 "PyFtpResult(outstanding: {:?}, profiles: {:?}, rate: {:?})",
//                 self.input_outstanding.as_ref(py).shape(),
//                 self.input_profiles.as_ref(py).shape(),
//                 self.input_rate.as_ref(py).shape()
//             )
//         })
//     }
//
//     fn __str__(&self) -> String {
//         self.__repr__()
//     }
//
//         /// Retourne la liste des attributs disponibles
//     fn __dir__(&self, py: Python<'_>) -> Vec<String> {
//         vec![
//             "input_outstanding".to_string(),
//             "input_profiles".to_string(),
//             "input_rate".to_string(),
//             "stock_amort".to_string(),
//             "stock_instal".to_string(),
//             "varstock_amort".to_string(), // or new prod amort
//             "varstock_instal".to_string(), // or new prod instal
//             "ftp_rate".to_string(),
//             "ftp_int".to_string(),
//             "market_rate".to_string(),
//         ]
//     }
//
// }
//

use crate::common::func_stock_instal::func_stock_instal;
use crate::common::func_stock_var_instal::func_stock_var_instal;
use crate::common::{
    func_ftp_int::func_ftp_int, func_ftp_rate::func_ftp_rate, func_market_rate::func_market_rate,
};
use crate::method_stock::{func_stock_amort::func_stock_amort, func_stock_var::func_stock_var};

impl FtpResult {
    fn check_dims(&self) {
        let (nrows_outs, ncols_outs) = self.input_outstanding.dim();
        let (nrows_profiles, ncols_profiles) = self.input_profiles.dim();
        let (nrows_rate, ncols_rate) = self.input_rate.dim();

        if (nrows_outs != nrows_profiles)
            || (nrows_outs != nrows_rate)
            || (nrows_profiles != nrows_rate)
        {
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

    pub fn new(
        input_outstanding: Array2<f64>,
        input_profiles: Array2<f64>,
        input_rate: Array2<f64>,
    ) -> Self {
        Self {
            input_outstanding,
            input_profiles,
            input_rate,
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

        if method == "stock" {
            // Implementation for "stock" method
            for i in 0..nrows {
                for j in 0..ncols {
                    func_stock_amort(self, i, j);
                    func_stock_instal(self, i, j);
                    func_stock_var(self, i, j, ncols);
                    func_stock_var_instal(self, i, j);
                }

                for j in (0..ncols).rev() {
                    if j > 0 {
                        func_ftp_rate(self, i, j - 1, ncols);
                        func_ftp_int(self, i, j - 1, ncols);
                        func_market_rate(self, i, j, ncols);
                    }
                }
            }
        } else if method == "flux" {
            // Implementation for "flux" method
            for i in 0..nrows {
                for j in 0..ncols {
                    flux_func_stock_var(self, i, j); // new prod
                    func_stock_var_instal(self, i, j);

                    flux_func_stock_amort(self, i, j);
                    func_stock_instal(self, i, j);
                }

                for j in (0..ncols).rev() {
                    if j > 0 {
                        func_ftp_rate(self, i, j - 1, ncols);
                        func_ftp_int(self, i, j - 1, ncols);
                        func_market_rate(self, i, j, ncols);
                    }
                }
            }
        } else {
            // Handle other methods or errors
        }
    }
}
