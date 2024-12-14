# quantrs

Technical Analysis Library for Trading in Rust

Quantrs is a Rust library that provides a comprehensive set of technical analysis tools for developing trading applications. It aims to mimic the functionality of the popular [TA-Lib](https://ta-lib.org/) library, offering high-performance implementations of various technical indicators.

This library is designed to seamlessly add technical analysis capabilities to your trading applications. It is heavily inspired by [ta-rs](https://github.com/gakonst/ta-rs) and aims to provide a robust alternative with a broader range of indicators.

## Features

- High-performance technical indicators implemented in Rust
- **Streaming functionality for real-time data processing**
- Extensible architecture for adding new indicators
- Open-source and community-driven development

## Installation

Add `quantrs` to your `Cargo.toml`:

```toml:Cargo.toml
[dependencies]
quantrs = "0.1.0"
```

Then, include it in your Rust code:

```rust:src/main.rs
use quantrs::prelude::*;
```

### Python bindings

Quantrs provides Python bindings via PyO3, allowing you to use the library in your Python projects. Install it using pip:

```bash
pip install quantrs
```

Example usage in Python:

```python
import quantrs

# Calculate SMA
data = [1.0, 2.0, 3.0, 4.0, 5.0]
sma_values = quantrs.sma(data, 3)
print(f"SMA values: {sma_values}")

# Use streaming functionality
sma_stream = quantrs.SMAStream(3)
for value in data:
    sma_value = sma_stream.next(value)
    print(f"New SMA value: {sma_value}")
```

## Usage

### Calculating the Simple Moving Average (SMA)

```rust:src/main.rs
use quantrs::trend::sma;

fn main() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let sma_values = sma(&data, 3);
    println!("SMA values: {:?}", sma_values);
}
```

### Streaming Example

Quantrs supports streaming data processing, allowing you to compute indicators on-the-fly as new data arrives, similar to TA-Lib's real-time functionalities.

```rust:src/main.rs
use quantrs::stream::SMAStream;

fn main() {
    let mut sma_stream = SMAStream::new(3);
    let data_stream = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    for value in data_stream {
        let sma_value = sma_stream.next(value);
        println!("New SMA value: {:?}", sma_value);
    }
}
```

## Available Indicators

Quantrs aims to provide a comprehensive set of technical indicators similar to those available in TA-Lib. The indicators are organized into categories for ease of use.

### Overlap Studies

- [x] SMA: Simple Moving Average
- [ ] EMA: Exponential Moving Average
- [ ] WMA: Weighted Moving Average
- [ ] DEMA: Double Exponential Moving Average
- [ ] TEMA: Triple Exponential Moving Average
- [ ] TRIMA: Triangular Moving Average
- [ ] KAMA: Kaufman Adaptive Moving Average
- [ ] MAMA: MESA Adaptive Moving Average
- [ ] T3: Triple Exponential Moving Average (T3)
- [ ] MACD: Moving Average Convergence/Divergence
- [ ] MACDEXT: MACD with controllable MA type
- [ ] MACDFIX: Moving Average Convergence/Divergence Fix 12/26
- [ ] BBANDS: Bollinger Bands
- [ ] MIDPOINT: MidPoint over period
- [ ] MIDPRICE: Midpoint Price over period
- [ ] SAR: Parabolic SAR
- [ ] SAREXT: Parabolic SAR - Extended

### Momentum Indicators

- [ ] ADX: Average Directional Movement Index
- [ ] ADXR: Average Directional Movement Index Rating
- [ ] APO: Absolute Price Oscillator
- [ ] AROON: Aroon
- [ ] AROONOSC: Aroon Oscillator
- [ ] BOP: Balance Of Power
- [ ] CCI: Commodity Channel Index
- [ ] CMO: Chande Momentum Oscillator
- [ ] DX: Directional Movement Index
- [ ] MFI: Money Flow Index
- [ ] MINUS_DI: Minus Directional Indicator
- [ ] MINUS_DM: Minus Directional Movement
- [ ] MOM: Momentum
- [ ] PLUS_DI: Plus Directional Indicator
- [ ] PLUS_DM: Plus Directional Movement
- [ ] PPO: Percentage Price Oscillator
- [ ] ROC: Rate of Change
- [ ] ROCP: Rate of Change Percentage
- [ ] ROCR: Rate of Change Ratio
- [ ] ROCR100: Rate of Change Ratio 100 Scale
- [ ] RSI: Relative Strength Index
- [ ] STOCH: Stochastic Oscillator
- [ ] STOCHF: Stochastic Fast
- [ ] STOCHRSI: Stochastic Relative Strength Index
- [ ] TRIX: Triple Exponential Average
- [ ] ULTOSC: Ultimate Oscillator
- [ ] WILLR: Williams' %R

### Volume Indicators

- [ ] AD: Chaikin A/D Line
- [ ] ADOSC: Chaikin A/D Oscillator
- [ ] OBV: On Balance Volume

### Volatility Indicators

- [ ] ATR: Average True Range
- [ ] NATR: Normalized Average True Range
- [ ] TRANGE: True Range

### Price Transform

- [ ] AVGPRICE: Average Price
- [ ] MEDPRICE: Median Price
- [ ] TYPPRICE: Typical Price
- [ ] WCLPRICE: Weighted Close Price

### Cycle Indicators

- [ ] HT_DCPERIOD: Hilbert Transform - Dominant Cycle Period
- [ ] HT_DCPHASE: Hilbert Transform - Dominant Cycle Phase
- [ ] HT_PHASOR: Hilbert Transform - Phasor Components
- [ ] HT_SINE: Hilbert Transform - SineWave
- [ ] HT_TRENDLINE: Hilbert Transform - Instantaneous Trendline
- [ ] HT_TRENDMODE: Hilbert Transform - Trend vs Cycle Mode

### Pattern Recognition

- [ ] CDL2CROWS: Two Crows
- [ ] CDL3BLACKCROWS: Three Black Crows
- [ ] CDL3INSIDE: Three Inside Up/Down
- *...and many more candlestick patterns*

### Statistical Functions

- [ ] BETA: Beta
- [ ] CORREL: Pearson's Correlation Coefficient (r)
- [ ] LINEARREG: Linear Regression
- [ ] LINEARREG_ANGLE: Linear Regression Angle
- [ ] LINEARREG_INTERCEPT: Linear Regression Intercept
- [ ] LINEARREG_SLOPE: Linear Regression Slope
- [ ] STDDEV: Standard Deviation
- [ ] TSF: Time Series Forecast
- [ ] VAR: Variance

### Math Transform

- [ ] ACOS: Vector Trigonometric ACos
- [ ] ASIN: Vector Trigonometric ASin
- [ ] ATAN: Vector Trigonometric ATan
- [ ] CEIL: Vector Ceiling
- [ ] COS: Vector Trigonometric Cos
- [ ] COSH: Vector Trigonometric Cosh
- [ ] EXP: Vector Natural Exponential Function
- [ ] FLOOR: Vector Floor
- [ ] LN: Vector Natural Logarithm
- [ ] LOG10: Vector Base-10 Logarithm
- [ ] SIN: Vector Trigonometric Sin
- [ ] SINH: Vector Trigonometric Sinh
- [ ] SQRT: Vector Square Root
- [ ] TAN: Vector Trigonometric Tan
- [ ] TANH: Vector Trigonometric Tanh

### Math Operators

- [ ] ADD: Vector Addition
- [ ] DIV: Vector Division
- [ ] MAX: Highest value over a specified period
- [ ] MIN: Lowest value over a specified period
- [ ] MINUS: Vector Subtraction
- [ ] MULT: Vector Multiplication
- [ ] SUM: Summation over a specified period

### Streaming Indicators

Quantrs provides streaming capabilities for real-time data processing. The streaming feature allows you to update indicator calculations efficiently as new data points become available, without recalculating the entire dataset.

- [ ] **SMAStream**: Streaming Simple Moving Average
- [ ] **EMAStream**: Streaming Exponential Moving Average
- [ ] **RSIStream**: Streaming Relative Strength Index
- *...and more streaming indicators to come*

*Note: This list includes a wide range of indicators provided by TA-Lib. Indicators will be implemented over time, and contributions are welcome!*

## Contributing

Contributions are welcome! If you'd like to help improve quantrs, please check out our [contribution guidelines](CONTRIBUTING.md) for more information.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [TA-Lib](https://ta-lib.org/)
- Heavily influenced by [ta-rs](https://github.com/gakonst/ta-rs)

## Contact

For questions or suggestions, feel free to open an issue on the [GitHub repository](https://github.com/quantrs-dev/quantrs).

---

*Happy Coding!*
