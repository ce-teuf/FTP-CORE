use ndarray::{Array2, ArrayBase, ViewRepr};
use ndarray::prelude::*;

pub fn extract_anti_diagonal_rect<'a, T>(arr: &'a ArrayBase<T, Ix2>) -> Vec<f64> 
where
    T: ndarray::Data<Elem = f64>,
{
    let (rows, cols) = arr.dim();
    let len = std::cmp::min(rows, cols);
    (0..len).map(|i| arr[(i, cols - 1 - i)].clone()).collect()
}

pub fn extract_anti_diagonal_rect2<'a, T>(arr: &'a ArrayBase<T, Ix2>) -> Vec<f64> 
where
    T: ndarray::Data<Elem = f64>,
{
    let (nrows, ncols) = arr.dim();
    // Create empty Vec<f64>
    let mut numbers: Vec<f64> = Vec::new();
    // numbers.push(3.14);
    if nrows < ncols {
        for i in 0..nrows {
            numbers.push(arr[[nrows-i-1, i]]);           
        }
    }
    else {
        for j in 0..nrows {
            if j < ncols {
                numbers.push(arr[[nrows-j-1, j]]);   
            }  
        }
    }
    
    numbers
    // let len = std::cmp::min(rows, cols);
    // (0..len).map(|i| arr[(i, cols - 1 - i)].clone()).collect()
}


pub fn print_array2(array2: &Array2<f64>) {
    // Print the second array
    println!("\n{}", array2);
}

pub fn get_shape(arr: &Array2<f64>, to_print: bool) -> (usize, usize) {
    let shape = arr.shape();
    if to_print {
        println!("Shape of the array: {:?}", shape);
    }

    // Convert the shape slice to a tuple
    (shape[0], shape[1])
}