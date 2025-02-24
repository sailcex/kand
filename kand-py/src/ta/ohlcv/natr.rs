use kand::{TAFloat, ohlcv::natr};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Normalized Average True Range (NATR) over NumPy arrays.
///
/// The NATR is a measure of volatility that accounts for the price level of the instrument.
/// It expresses the ATR as a percentage of the closing price.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   high: High prices as a 1-D NumPy array of type `f32`.
///   low: Low prices as a 1-D NumPy array of type `f32`.
///   close: Close prices as a 1-D NumPy array of type `f32`.
///   period: Window size for NATR calculation. Must be positive and less than input length.
///
/// Returns:
///   A new 1-D NumPy array containing the NATR values. The array has the same length as the input,
///   with the first `period` elements containing NaN values.
///
/// Note:
///   This function releases the Python GIL during computation using `py.allow_threads()` to enable
///   concurrent Python execution.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
///   >>> low = np.array([8.0, 9.0, 11.0, 10.0, 9.0])
///   >>> close = np.array([9.0, 11.0, 14.0, 12.0, 11.0])
///   >>> result = kand.natr(high, low, close, 3)
///   ```
#[pyfunction]
#[pyo3(name = "natr", signature = (high, low, close, period))]
pub fn natr_py(
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

    // Perform the NATR calculation while releasing the GIL to allow other Python threads to run
    py.allow_threads(|| {
        natr::natr(
            input_high,
            input_low,
            input_close,
            period,
            output.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output array to a Python object
    Ok(output.into_pyarray(py).into())
}

/// Calculates the next NATR value incrementally.
///
/// This function provides an optimized way to calculate a single new NATR value
/// using the previous ATR value and current price data, without recalculating the entire series.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   high: Current period's high price.
///   low: Current period's low price.
///   close: Current period's closing price.
///   prev_close: Previous period's closing price.
///   prev_atr: Previous period's ATR value.
///   period: Period for NATR calculation (must be >= 2).
///
/// Returns:
///   The calculated NATR value.
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> natr = kand.natr_incremental(
///   ...     15.0,  # high
///   ...     11.0,  # low
///   ...     14.0,  # close
///   ...     12.0,  # prev_close
///   ...     3.0,   # prev_atr
///   ...     3      # period
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "natr_incremental", signature = (high, low, close, prev_close, prev_atr, period))]
pub fn natr_incremental_py(
    py: Python,
    high: TAFloat,
    low: TAFloat,
    close: TAFloat,
    prev_close: TAFloat,
    prev_atr: TAFloat,
    period: usize,
) -> PyResult<TAFloat> {
    // Perform the incremental NATR calculation while releasing the GIL
    py.allow_threads(|| natr::natr_incremental(high, low, close, prev_close, prev_atr, period))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
