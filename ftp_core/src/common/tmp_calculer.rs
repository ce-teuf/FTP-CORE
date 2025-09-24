pub fn cus_addition(a: f64, b: f64) -> f64 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cus_addition_positive_numbers() {
        assert_eq!(cus_addition(2.5, 3.7), 6.2);
    }

    #[test]
    fn test_cus_addition_negative_numbers() {
        assert_eq!(cus_addition(-5.0, 3.0), -2.0);
    }

    #[test]
    fn test_cus_addition_zero() {
        assert_eq!(cus_addition(0.0, 0.0), 0.0);
        assert_eq!(cus_addition(5.0, 0.0), 5.0);
    }

    #[test]
    fn test_cus_addition_large_numbers() {
        assert_eq!(cus_addition(1e10, 1e10), 2e10);
    }
}