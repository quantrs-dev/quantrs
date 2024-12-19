//! Exponential Moving Average
//! # Examples
//!
//! ```
//! use quantrs::trend::ema;
//! let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! let ema = ema(&data, 3).unwrap();
//! let expected = vec![1.0, 1.5, 2.25, 3.125, 4.0625];
//! assert!(ema.iter().zip(expected.iter()).all(|(a, b)| {
//!     (a - b).abs() < 1e-10
//! }));
//! ```

use crate::error::{QuantrsError, Result};
use ndarray::Array1;

pub fn ema(data: &[f64], period: usize) -> Result<Vec<f64>> {
    if period == 0 {
        return Err(QuantrsError::InvalidPeriod);
    }

    let mut ema = vec![f64::NAN; data.len()];
    if data.is_empty() {
        return Ok(ema);
    }

    let multiplier = 2.0 / (period as f64 + 1.0);
    let data_array = Array1::from(data.to_vec());

    // Initialize the first value
    ema[0] = data[0];
    for i in 1..data.len() {
        ema[i] = (data_array[i] - ema[i - 1]) * multiplier + ema[i - 1];
    }

    Ok(ema)
}

pub struct EMAStream {
    period: usize,
    multiplier: f64,
    ema: Option<f64>,
}

impl EMAStream {
    pub fn period(&self) -> usize {
        self.period
    }

    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(QuantrsError::InvalidPeriod),
            _ => Ok(Self {
                period,
                multiplier: 2.0 / (period as f64 + 1.0),
                ema: None,
            }),
        }
    }

    pub fn next(&mut self, value: f64) -> f64 {
        self.ema = match self.ema {
            Some(prev_ema) => Some((value - prev_ema) * self.multiplier + prev_ema),
            None => Some(value),
        };
        self.ema.unwrap()
    }
}
