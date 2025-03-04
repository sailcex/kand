use kand::{TAFloat, ohlcv::trima};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculate Triangular Moving Average (TRIMA) for a NumPy array.
///
/// TRIMA is a double-smoothed moving average that places more weight on the middle portion of the price series
/// and less weight on the first and last portions. This results in a smoother moving average compared to a
/// Simple Moving Average (SMA).
///
/// Args:
///   prices: Input prices as a 1-D NumPy array of type `TAFloat`.
///   period: Smoothing period for calculations (must be >= 2).
///
/// Returns:
///   A tuple of 2 1-D NumPy arrays containing:
///   - First SMA values
///   - Final TRIMA values
///   The first (period - 1) elements of each array contain NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> prices = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
///   >>> sma1, trima = kand.trima(prices, 3)
///   ```
#[pyfunction]
#[pyo3(name = "trima", signature = (prices, period))]
pub fn trima_py(
    py: Python,
    prices: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<(Py<PyArray1<TAFloat>>, Py<PyArray1<TAFloat>>)> {
    let prices_slice = prices.as_slice()?;
    let len = prices_slice.len();

    let mut output_sma1 = vec![0.0; len];
    let mut output_sma2 = vec![0.0; len];

    py.allow_threads(|| {
        trima::trima(
            prices_slice,
            period,
            output_sma1.as_mut_slice(),
            output_sma2.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output_sma1.into_pyarray(py).into(),
        output_sma2.into_pyarray(py).into(),
    ))
}

/// Calculate the next TRIMA value incrementally.
///
/// Args:
///   prev_sma1: Previous first SMA value.
///   prev_sma2: Previous TRIMA value.
///   new_price: Latest price to include in calculation.
///   old_price: Price dropping out of first window.
///   old_sma1: SMA1 value dropping out of second window.
///   period: Smoothing period for calculations (must be >= 2).
///
/// Returns:
///   A tuple containing:
///   - Updated first SMA value
///   - Updated TRIMA value
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> trima, sma1 = kand.trima_inc(
///   ...     35.5,  # prev_sma1
///   ...     35.2,  # prev_sma2
///   ...     36.0,  # new_price
///   ...     35.0,  # old_price
///   ...     35.1,  # old_sma1
///   ...     5      # period
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "trima_inc", signature = (prev_sma1, prev_sma2, new_price, old_price, old_sma1, period))]
pub fn trima_inc_py(
    prev_sma1: TAFloat,
    prev_sma2: TAFloat,
    new_price: TAFloat,
    old_price: TAFloat,
    old_sma1: TAFloat,
    period: usize,
) -> PyResult<(TAFloat, TAFloat)> {
    trima::trima_inc(prev_sma1, prev_sma2, new_price, old_price, old_sma1, period)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
