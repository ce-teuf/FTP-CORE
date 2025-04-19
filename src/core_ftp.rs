use ndarray::{Array2, array, s};

use crate::utils::print_array2;
use crate::utils::extract_anti_diagonal_rect2;
use crate::utils::extract_anti_diagonal_rect;

pub fn rebuilt_market_rate(v_encours: &Array2<f64>, v_ftp_rate: &Array2<f64>) -> Array2<f64> {
    // v_encours : (1, m) & v_ftp_rate : (1, m-1)
    let (nrows, ncols) = v_encours.dim();
    let mut res = Array2::<f64>::zeros((nrows, ncols));

    // diff encours
    let mut diff_encours = Array2::<f64>::zeros((nrows, ncols));
    for i in 1..ncols {
        diff_encours[[0, i]] = v_encours[[0, i-1]] - v_encours[[0, i]];
    }

    // init
    res[[0, ncols-1]] = v_ftp_rate[[0, ncols-1-1]];
    
    // iter
    for i in (1..=ncols-1-1).rev().step_by(1) {
        //println!("---- i: {}", i); 

        let mut comp1: f64  = diff_encours.slice(s![0, i..ncols]).iter().sum();
        let mut comp2: f64 = 0.0;
        let mut comp3: f64 = v_ftp_rate[[0, i-1]];
        let mut comp4: f64 = diff_encours[[0, i]];
        
        //res[[0, i]] = ;
        let mut rt;
        for k in (i+1..ncols).rev().step_by(1) {
            //println!("-- k: {}", k);
            rt = res[[0, k]] * diff_encours[[0, k]] * -1.0;
            //println!("-- rt: {}", rt);
            comp2 += rt;
        }

        //println!("comp1: {}", comp1);
        //println!("comp2: {}", comp2);
        //println!("comp3: {}", comp3);
        //println!("comp4: {}", comp4);
        //println!("res: {}", ((comp3*(comp1))+(comp2))/comp4  );

        res[[0, i]] =((comp3*(comp1))+(comp2))/comp4 ;
    }
    res
}



pub fn m_step_5_ftp_rate(m_rate: &Array2<f64>, m_si: &Array2<f64>) -> Array2<f64> {
    
    let (nrows, ncols) = m_rate.dim();
    let mut res = Array2::<f64>::zeros((nrows, ncols));

    for i in 0..nrows {
        for j in 0..ncols {
            //println!("----- {} - {} ------", i, j);
            let mut denom = 0.0;
            let mut nom = 0.0;
            for k in 0..i+1 {
                if i-k+j <= ncols-1 {
                    //println!("{}, {}:{}", k, i-k+j, ncols-1);
                    // denom stack
                    let aa = m_si.slice(s![k, i-k+j+1..ncols+1]);
                    let bb: f64 = aa.iter().sum();
                    denom += bb;
                    //println!("++{}++", bb);
                    // nom stack
                }
                
            }
            //println!("denom : {}", denom);
        }
    }
    res
}

pub fn m_step_4_stockvar_instalment(matrix: &Array2<f64>) -> Array2<f64> {
    
    let (nrows, ncols) = matrix.dim();
    let mut res = Array2::<f64>::zeros((nrows, ncols));

    for i in 0..matrix.nrows() {
        for j in 1..matrix.ncols() {
            res[[i, j]] = matrix[[i, j-1]] - matrix[[i, j]];
        }
    }

    res
}

pub fn m_step_3_stockvar(matrix: &Array2<f64>) -> Array2<f64> {
    
    let (nrows, ncols) = matrix.dim();
    let mut res = Array2::<f64>::zeros((nrows, ncols));

    // init la premiere ligne 
    for j in 0..ncols {
        res[[0, j]] = matrix[[0, j]]
    }

    for i in 1..nrows {
        for j in 0..ncols-1 {

            let mut value = matrix[(i, j)];
            
            let subarray = res.slice(s![0..i, j+1..ncols]);
            // print_array2(&subarray.to_owned());
            let g;
            g = extract_anti_diagonal_rect2(&subarray);
            // println!("{:?}", g);
            
            let sumx: f64 = g.iter().sum();
            // println!("{:?}", sumx);
            res[(i, j)] = value - sumx;
            // println!("{:?}", value - sumx);
        }
    }
    res
}


pub fn m_step_2_outstanding_instalment(matrix: &Array2<f64>) -> Array2<f64> {
    
    let (nrows, ncols) = matrix.dim();
    let mut res = Array2::<f64>::zeros((nrows, ncols));

    for i in 0..matrix.nrows() {
        for j in 1..matrix.ncols() {
            res[[i, j]] = matrix[[i, j-1]] - matrix[[i, j]];
        }
    }

    res
}

pub fn m_step_1_outstanding_amort(vector: &Array2<f64>, matrix: &Array2<f64>) -> Array2<f64> {
    
    assert_eq!(vector.nrows(), matrix.nrows(), "Vector and matrix dimensions are incompatible for element-wise multiplication.");
    assert_eq!(vector.ncols(), 1, "Vector should have a single column.");

    let (nrows, ncols) = matrix.dim();
    let mut res = Array2::<f64>::zeros((nrows, ncols));

    for i in 0..matrix.nrows() {
        let a = vector[[i, 0]];
        for j in 0..matrix.ncols() {
            res[[i, j]] = a * matrix[[i, j]];
        }
    }
    
    res
}

