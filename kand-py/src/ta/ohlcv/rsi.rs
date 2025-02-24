use kand::{TAFloat, ohlcv::rsi};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Relative Strength Index (RSI) over NumPy arrays.
///
/// The RSI is a momentum oscillator that measures the speed and magnitude of recent price changes
/// to evaluate overbought or oversold conditions.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   prices: Input prices as a 1-D NumPy array of type `f64`.
///   period: Window size for RSI calculation. Must be positive and less than input length.
///
/// Returns:
///   A tuple of three 1-D NumPy arrays containing:
///   - RSI values
///   - Average gain values
///   - Average loss values
///   Each array has the same length as the input, with the first `period` elements containing NaN values.
///
/// Note:
///   This function releases the Python GIL during computation using `py.allow_threads()` to enable
///   concurrent Python execution.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> prices = np.array([44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42])
///   >>> rsi, avg_gain, avg_loss = kand.rsi(prices, 5)
///   ```
#[pyfunction]
#[pyo3(name = "rsi", signature = (prices, period))]
pub fn rsi_py(
    py: Python,
    prices: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    // Convert the input NumPy array to a Rust slice
    let input_prices = prices.as_slice()?;
    let len = input_prices.len();

    // Create new output arrays using vec
    let mut output_rsi = vec![0.0; len];
    let mut output_avg_gain = vec![0.0; len];
    let mut output_avg_loss = vec![0.0; len];

    // Perform the RSI calculation while releasing the GIL to allow other Python threads to run
    py.allow_threads(|| {
        rsi::rsi(
            input_prices,
            period,
            output_rsi.as_mut_slice(),
            output_avg_gain.as_mut_slice(),
            output_avg_loss.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output arrays to Python objects
    Ok((
        output_rsi.into_pyarray(py).into(),
        output_avg_gain.into_pyarray(py).into(),
        output_avg_loss.into_pyarray(py).into(),
    ))
}

/// Calculates a single RSI value incrementally.
///
/// This function provides an optimized way to calculate the latest RSI value
/// when streaming data is available, without needing the full price history.
///
/// Args:
///   current_price: The current period's price value.
///   prev_price: The previous period's price value.
///   prev_avg_gain: The previous period's average gain.
///   prev_avg_loss: The previous period's average loss.
///   period: The time period for RSI calculation.
///
/// Returns:
///   A tuple containing (RSI value, new average gain, new average loss).
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> rsi, avg_gain, avg_loss = kand.rsi_incremental(45.42, 45.10, 0.24, 0.14, 14)
///   ```
#[pyfunction]
#[pyo3(name = "rsi_incremental", signature = (current_price, prev_price, prev_avg_gain, prev_avg_loss, period))]
pub fn rsi_incremental_py(
    current_price: TAFloat,
    prev_price: TAFloat,
    prev_avg_gain: TAFloat,
    prev_avg_loss: TAFloat,
    period: usize,
) -> PyResult<(TAFloat, TAFloat, TAFloat)> {
    rsi::rsi_incremental(
        current_price,
        prev_price,
        prev_avg_gain,
        prev_avg_loss,
        period,
    )
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
