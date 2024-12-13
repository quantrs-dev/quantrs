pub mod prelude {
    pub use crate::averages::*;
}

// Feature-gated indicators by category
#[cfg(feature = "averages")]
pub mod averages {
    //! Moving averages and related indicators
    //!
    //! # Examples
    //!
    //! ```
    //! use quantrs::averages::sma;
    //! let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    //! let sma = sma(&data, 3);
    //! assert_eq!(sma, vec![2.0, 3.0, 4.0]);
    //! ```
    mod sma;
    pub use sma::*;
}

#[cfg(feature = "momentum")]
pub mod momentum {
    //! Momentum-based indicators
}

#[cfg(feature = "volatility")]
pub mod volatility {
    //! Volatility-based indicators
}

#[cfg(feature = "volume")]
pub mod volume {
    //! Volume-based indicators
}
