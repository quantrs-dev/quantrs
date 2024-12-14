//! Simple Moving Average
//! # Examples
//!
//! ```
//! use quantrs::trend::sma;
//! let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! let sma = sma(&data, 3);
//! let expected = vec![f64::NAN, f64::NAN, 2.0, 3.0, 4.0];
//! assert!(sma.iter().zip(expected.iter()).all(|(a, b)| {
//!     if a.is_nan() && b.is_nan() {
//!         true
//!     } else {
//!         (a - b).abs() < 1e-10
//!     }
//! }));
//! ```
pub fn sma(data: &[f64], period: usize) -> Vec<f64> {
    if period == 0 {
        panic!("Period must be greater than 0");
    }

    let mut sma = vec![f64::NAN; data.len()];

    // Handle empty data case
    if data.is_empty() {
        return sma;
    }

    // Special case for period 1
    if period == 1 {
        return data.to_vec();
    }

    // If period is larger than data length, return all NaN
    if period > data.len() {
        return sma;
    }

    // Calculate SMA starting from index period-1
    for i in period - 1..data.len() {
        let sum: f64 = data[i + 1 - period..=i].iter().sum();
        sma[i] = sum / period as f64;
    }

    sma
}
