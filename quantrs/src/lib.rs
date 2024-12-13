pub mod prelude {
    pub use crate::averages::*;
}

// Feature-gated indicators by category
#[cfg(feature = "averages")]
pub mod averages {
    //! Moving averages and related indicators
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
