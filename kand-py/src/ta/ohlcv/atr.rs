use kand::{TAFloat, ohlcv::atr};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Average True Range (ATR) over NumPy arrays.
///
/// The Average True Range (ATR) is a technical analysis indicator that measures market volatility
/// by decomposing the entire range of an asset price for a given period.
///
/// Args:
///   high: High prices as a 1-D NumPy array of type `TAFloat`.
///   low: Low prices as a 1-D NumPy array of type `TAFloat`.
///   close: Close prices as a 1-D NumPy array of type `TAFloat`.
///   period: Window size for ATR calculation. Must be greater than 1.
///
/// Returns:
///   A new 1-D NumPy array containing the ATR values. The array has the same length as the input,
///   with the first `period` elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
///   >>> low = np.array([8.0, 9.0, 11.0, 10.0, 9.0])
///   >>> close = np.array([9.0, 11.0, 14.0, 12.0, 11.0])
///   >>> result = kand.atr(high, low, close, 3)
///   ```
#[pyfunction]
#[pyo3(name = "atr", signature = (high, low, close, period))]
pub fn atr_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    // Convert the input NumPy arrays to Rust slices
    let input_high = high.as_slice()?;
    let input_low = low.as_slice()?;
    let input_close = close.as_slice()?;
    let len = input_high.len();

    // Create a new output array using vec
    let mut output = vec![0.0; len];

    // Perform the ATR calculation while releasing the GIL to allow other Python threads to run
    py.allow_threads(|| {
        atr::atr(
            input_high,
            input_low,
            input_close,
            period,
            output.as_mut_slice(),
        )
    })
    .map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error computing ATR: {:?}", e))
    })?;

    // Convert the output array to a Python object
    Ok(output.into_pyarray(py).into())
}

/// Calculate the next ATR value incrementally.
///
/// Args:
///
///   high: Current period's high price.
///   low: Current period's low price.
///   prev_close: Previous period's close price.
///   prev_atr: Previous period's ATR value.
///   period: The time period for ATR calculation (must be >= 2).
///
/// Returns:
///   The calculated ATR value.
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> atr = kand.atr_incremental(
///   ...     15.0,  # high
///   ...     11.0,  # low
///   ...     12.0,  # prev_close
///   ...     3.0,   # prev_atr
///   ...     14     # period
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "atr_incremental", signature = (high, low, prev_close, prev_atr, period))]
pub fn atr_incremental_py(
    py: Python,
    high: TAFloat,
    low: TAFloat,
    prev_close: TAFloat,
    prev_atr: TAFloat,
    period: usize,
) -> PyResult<TAFloat> {
    py.allow_threads(|| atr::atr_incremental(high, low, prev_close, prev_atr, period))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
