//! Simple Moving Average
//! # Examples
//!
//! ```
//! use quantrs::averages::sma;
//! let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! let sma = sma(&data, 3);
//! let expected = vec![0.0, 0.0, 0.0, 2.0, 3.0];
//! assert_eq!(sma, expected);
//! ```
pub fn sma(data: &[f64], period: usize) -> Vec<f64> {
    let mut sma = vec![0.0; data.len()];
    for i in period..data.len() {
        sma[i] = data[i - period..i].iter().sum::<f64>() / period as f64;
    }
    sma
}
