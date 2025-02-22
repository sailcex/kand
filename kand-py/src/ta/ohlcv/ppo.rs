use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use kand::ohlcv::ppo;
use kand::TAFloat;

/// Computes the Percentage Price Oscillator (PPO) over NumPy arrays.
///
/// The PPO is a momentum oscillator that measures the percentage difference between two moving averages.
/// It is similar to the MACD but expresses values as percentages.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   data: Input price data as a 1-D NumPy array of type `f32`.
///   fast_period: Fast moving average period. Must be >= 2.
///   slow_period: Slow moving average period. Must be >= 2 and > fast_period.
///   ma_type: Type of moving average to use ("sma" or "ema").
///
/// Returns:
///   A tuple of three 1-D NumPy arrays containing:
///   - PPO values
///   - Fast moving average values
///   - Slow moving average values
///   Each array has the same length as the input, with initial elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> data = np.array([10.0, 11.0, 12.0, 13.0, 14.0])
///   >>> ppo, fast_ma, slow_ma = kand.ppo(data, 2, 3, "sma")
///   ```
#[pyfunction]
#[pyo3(name = "ppo", signature = (data, fast_period, slow_period, ma_type))]
pub fn ppo_py(
    py: Python,
    data: PyReadonlyArray1<TAFloat>,
    fast_period: usize,
    slow_period: usize,
    ma_type: &str,
) -> PyResult<(Py<PyArray1<TAFloat>>, Py<PyArray1<TAFloat>>, Py<PyArray1<TAFloat>>)> {
    // Convert the input NumPy array to a Rust slice
    let input = data.as_slice()?;
    let len = input.len();

    // Create new output arrays using vec
    let mut output_ppo = vec![0.0; len];
    let mut output_fast_ma = vec![0.0; len];
    let mut output_slow_ma = vec![0.0; len];

    // Convert MA type string to enum
    let ma_type = match ma_type.to_lowercase().as_str() {
        "sma" => ta::utils::types::MAType::SMA,
        "ema" => ta::utils::types::MAType::EMA,
        _ => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Invalid MA type",
            ))
        }
    };

    // Perform the PPO calculation while releasing the GIL to allow other Python threads to run
    py.allow_threads(|| {
        ppo::ppo(
            input,
            fast_period,
            slow_period,
            ma_type,
            output_ppo.as_mut_slice(),
            output_fast_ma.as_mut_slice(),
            output_slow_ma.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output arrays to Python objects
    Ok((
        output_ppo.into_pyarray(py).into(),
        output_fast_ma.into_pyarray(py).into(),
        output_slow_ma.into_pyarray(py).into(),
    ))
}

/// Calculates the next PPO value incrementally using previous values.
///
/// This function enables real-time calculation of PPO by using previous moving average values
/// and current price data, avoiding the need to recalculate the entire series.
///
/// Args:
///   price: Current price value as `f32`.
///   prev_fast_ma: Previous fast moving average value as `f32`.
///   prev_slow_ma: Previous slow moving average value as `f32`.
///   old_fast_price: Price to remove from fast MA (for SMA only) as `f32`.
///   old_slow_price: Price to remove from slow MA (for SMA only) as `f32`.
///   fast_period: Fast moving average period (>= 2).
///   slow_period: Slow moving average period (>= 2 and > fast_period).
///   ma_type: Type of moving average to use ("sma" or "ema").
///
/// Returns:
///   A tuple containing (latest PPO, new fast MA, new slow MA).
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> ppo, fast_ma, slow_ma = kand.ppo_incremental(
///   ...     100.0,  # current price
///   ...     98.5,   # previous fast MA
///   ...     99.0,   # previous slow MA
///   ...     0.0,    # old fast price (unused for EMA)
///   ...     0.0,    # old slow price (unused for EMA)
///   ...     12,     # fast period
///   ...     26,     # slow period
///   ...     "ema"   # MA type
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "ppo_incremental", signature = (price, prev_fast_ma, prev_slow_ma, old_fast_price, old_slow_price, fast_period, slow_period, ma_type))]
pub fn ppo_incremental_py(
    price: TAFloat,
    prev_fast_ma: TAFloat,
    prev_slow_ma: TAFloat,
    old_fast_price: TAFloat,
    old_slow_price: TAFloat,
    fast_period: usize,
    slow_period: usize,
    ma_type: &str,
) -> PyResult<(TAFloat, TAFloat, TAFloat)> {
    // Convert MA type string to enum
    let ma_type = match ma_type.to_lowercase().as_str() {
        "sma" => ta::utils::types::MAType::SMA,
        "ema" => ta::utils::types::MAType::EMA,
        _ => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Invalid MA type",
            ))
        }
    };

    ppo::ppo_incremental(
        price,
        prev_fast_ma,
        prev_slow_ma,
        old_fast_price,
        old_slow_price,
        fast_period,
        slow_period,
        ma_type,
    )
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
