//! Simple Moving Average
//! # Examples
//!
//! ```
//! use quantrs::trend::sma;
//! let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! let sma = sma(&data, 3).unwrap();
//! let expected = vec![f64::NAN, f64::NAN, 2.0, 3.0, 4.0];
//! assert!(sma.iter().zip(expected.iter()).all(|(a, b)| {
//!     if a.is_nan() && b.is_nan() {
//!         true
//!     } else {
//!         (a - b).abs() < 1e-10
//!     }
//! }));
//! ```
use crate::error::{QuantrsError, Result};
use ndarray::s;
use ndarray::Array1;
use std::collections::VecDeque;

pub fn sma(data: &[f64], period: usize) -> Result<Vec<f64>> {
    if period == 0 {
        return Err(QuantrsError::InvalidPeriod);
    }

    let mut sma = vec![f64::NAN; data.len()];

    // Handle empty data case
    // and period is larger than data length, return all NaN
    if data.is_empty() || period > data.len() {
        return Ok(sma);
    }

    // Special case for period 1
    if period == 1 {
        return Ok(data.to_vec());
    }

    let data_array = Array1::from(data.to_vec());

    // Calculate SMA starting from index period-1
    for i in 0..(data.len() - period + 1) {
        let window = data_array.slice(s![i..i + period]);
        let sum: f64 = window.sum();
        sma[i + period - 1] = sum / period as f64;
    }

    Ok(sma)
}

pub struct SMAStream {
    data: VecDeque<f64>,
    period: usize,
    sum: f64,
}

impl SMAStream {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(QuantrsError::InvalidPeriod),
            _ => Ok(Self {
                data: VecDeque::with_capacity(period),
                period,
                sum: 0.0,
            }),
        }
    }

    pub fn next(&mut self, value: f64) -> Option<f64> {
        self.sum += value;
        self.data.push_back(value);
        if self.data.len() > self.period {
            self.sum -= self.data.pop_front().unwrap();
        }
        let is_ready = self.data.len() >= self.period;
        is_ready.then(|| self.sum / self.period as f64)
    }
}
