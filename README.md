<h1 align="center">
  <img src="docs/assets/logo.png" alt="Kand Logo" width="400">
</h1>

<div align="center">
  <a href="https://crates.io/crates/kand">
    <img src="https://img.shields.io/crates/v/kand.svg" alt="Crates.io"/>
  </a>
  <a href="https://docs.rs/kand">
    <img src="https://docs.rs/kand/badge.svg" alt="Docs.rs"/>
  </a>
  <a href="https://pypi.python.org/pypi/kand">
    <img src="https://img.shields.io/pypi/v/kand.svg" alt="PyPI Version"/>
  </a>
  <a href="https://pypi.python.org/pypi/kand">
    <img src="https://img.shields.io/pypi/pyversions/kand.svg" alt="Python Versions"/>
  </a>
  <a href="https://github.com/rust-ta/kand/actions/workflows/CI.yml">
    <img src="https://github.com/rust-ta/kand/actions/workflows/CI.yml/badge.svg" alt="CI Status"/>
  </a>
  <a href="https://github.com/rust-ta/kand/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/rust-ta/kand" alt="License"/>
  </a>
</div>
<p align="center">
  <b>Documentation</b>:
  <a href="https://docs.rs/kand">Rust</a>
  -
  <a href="https://rust-ta.github.io/kand/">Python</a>
  |
  <b>Repository</b>:
  <a href="https://github.com/rust-ta/kand">GitHub</a>
</p>
<h2 align="center">
  <b>Kand: Blazingly Fast Technical Analysis Library in Rust, Python</b>
</h2>


## Features
- Zero-cost abstractions and SIMD optimizations
- Thread-safe and memory-safe by design
- Comprehensive test coverage and documentation
- Clean, ergonomic API with robust error handling

## Setup

### Python

Install the latest Kand version with:

```bash
pip install kand
```

### Rust

You can take latest release from `crates.io`, or if you want to use the latest features / performance improvements point to the `main` branch of this repo.

```toml
[dependencies]
kand = { git = "https://github.com/rust-ta/kand", rev = "<optional git tag>" }
```

Recommend Rust version `>=1.80`.

## Functions List

### OHLCV Based

- [x] **AD** - Chaikin A/D Line
- [x] **ADOSC** - Chaikin A/D Oscillator
- [x] **ADR** - Average Daily Range **[Untested]**
- [x] **ADX** - Average Directional Movement Index
- [x] **ADXR** - Average Directional Movement Index Rating
- [ ] **APO** - Absolute Price Oscillator
- [x] **AROON** - Aroon
- [x] **AROONOSC** - Aroon Oscillator
- [x] **ATR** - Average True Range
- [x] **BBANDS** - Bollinger Bands
- [x] **BOP** - Balance Of Power
- [x] **CCI** - Commodity Channel Index
- [x] **CDL_DOJI** - Doji
- [x] **CDL_DRAGONFLY_DOJI** - Dragonfly Doji
- [x] **CDL_GRAVESTONE_DOJI** - Gravestone Doji
- [x] **CDL_HAMMER** - Hammer
- [x] **CDL_INVERTED_HAMMER** - Inverted Hammer
- [x] **CDL_LONG_LOWER_SHADOW** - Long Lower Shadow
- [x] **CDL_LONG_UPPER_SHADOW** - Long Upper Shadow
- [x] **CDL_MARUBOZU** - Marubozu
- [ ] **CMO** - Chande Momentum Oscillator
- [x] **DEMA** - Double Exponential Moving Average
- [x] **DX** - Directional Movement Index
- [x] **EMA** - Exponential Moving Average
- [x] **ECL** - Expanded Camarilla Levels **[Untested]**
- [x] **HA** - Heikin Ashi Chart
- [ ] **HT_DCPERIOD** - Hilbert Transform - Dominant Cycle Period
- [ ] **HT_DCPHASE** - Hilbert Transform - Dominant Cycle Phase
- [ ] **HT_PHASOR** - Hilbert Transform - Phasor Components
- [ ] **HT_SINE** - Hilbert Transform - SineWave
- [ ] **HT_TRENDLINE** - Hilbert Transform - Instantaneous Trendline
- [ ] **HT_TRENDMODE** - Hilbert Transform - Trend vs Cycle Mode
- [ ] **KAMA** - Kaufman Adaptive Moving Average
- [ ] **LINEARREG** - Linear Regression
- [ ] **LINEARREG_ANGLE** - Linear Regression Angle
- [ ] **LINEARREG_INTERCEPT** - Linear Regression Intercept
- [ ] **LINEARREG_SLOPE** - Linear Regression Slope
- [x] **MACD** - Moving Average Convergence/Divergence **[Unstable]**
- [ ] **MACDEXT** - MACD with controllable MA type
- [ ] **MAMA** - MESA Adaptive Moving Average
- [x] **MEDPRICE** - Median Price
- [x] **MFI** - Money Flow Index **[No Incremental]**
- [x] **MIDPOINT** - MidPoint over period
- [x] **MIDPRICE** - Midpoint Price over period
- [x] **MINUS_DI** - Minus Directional Indicator
- [x] **MINUS_DM** - Minus Directional Movement
- [x] **MOM** - Momentum
- [x] **NATR** - Normalized Average True Range
- [x] **OBV** - On Balance Volume
- [x] **PLUS_DI** - Plus Directional Indicator
- [x] **PLUS_DM** - Plus Directional Movement
- [ ] **PPO** - Percentage Price Oscillator
- [ ] **RENKO** - Renko Chart
- [x] **RMA** - Rolling Moving Average **[Untested]**
- [x] **ROC** - Rate of change : ((price/prevPrice)-1)*100
- [x] **ROCP** - Rate of change Percentage: (price-prevPrice)/prevPrice
- [x] **ROCR** - Rate of change ratio: (price/prevPrice)
- [x] **ROCR100** - Rate of change ratio 100 scale: (price/prevPrice)*100
- [x] **RSI** - Relative Strength Index
- [x] **SAR** - Parabolic SAR
- [ ] **SAREXT** - Parabolic SAR - Extended
- [x] **SMA** - Simple Moving Average
- [x] **STOCH** - Stochastic **[No Incremental]**
- [ ] **STOCHF** - Stochastic Fast
- [ ] **STOCHRSI** - Stochastic Relative Strength Index
- [x] **SUPERTREND** - Super Trend Indicator
- [x] **T3** - Triple Exponential Moving Average (T3)
- [x] **TEMA** - Triple Exponential Moving Average
- [x] **TPO** - Time Price Opportunity
- [x] **TRANGE** - True Range
- [x] **TRIMA** - Triangular Moving Average
- [x] **TRIX** - 1-day Rate-Of-Change (ROC) of a Triple Smooth EMA
- [ ] **TSF** - Time Series Forecast
- [x] **TYPPRICE** - Typical Price
- [ ] **ULTOSC** - Ultimate Oscillator
- [x] **VEGAS** - VEGAS Channel and Trend Boundary EMAs **[Untested]**
- [x] **VWAP** - Volume Weighted Average Price
- [x] **WCLPRICE** - Weighted Close Price
- [x] **WILLR** - Williams' %R
- [x] **WMA** - Weighted Moving Average

### Statistical Analysis

- [ ] **ALPHA** - Alpha: Measures excess returns over market
- [ ] **BETA** - Beta: Measures sensitivity to market volatility
- [ ] **CALMAR** - Calmar Ratio: Annual return to maximum drawdown ratio
- [ ] **CORREL** - Pearson's Correlation Coefficient
- [ ] **DRAWDOWN** - Maximum Drawdown: Maximum potential loss
- [ ] **KELLY** - Kelly Criterion: Optimal position sizing
- [x] **MAX** - Highest value over a specified period
- [x] **MIN** - Lowest value over a specified period
- [ ] **SHARPE** - Sharpe Ratio: Risk-adjusted return measure
- [ ] **SORTINO** - Sortino Ratio: Downside risk-adjusted returns
- [x] **STDDEV** - Standard Deviation
- [x] **SUM** - Summation
- [x] **VAR** - Variance
- [ ] **WINRATE** - Win Rate: Strategy success probability

## Contributing

We are passionate about supporting contributors of all levels of experience and would love to see
you get involved in the project. See the
[contributing guide](https://github.com/rust-ta/kand/blob/main/CONTRIBUTING.md) to get started.


## License

This project is licensed under either of the following licenses, at your option:
- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in kand by you, as defined in the Apache-2.0 license, shall be dually licensed as above, without any additional terms or conditions.
