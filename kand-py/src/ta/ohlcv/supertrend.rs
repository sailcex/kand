use kand::{ohlcv::supertrend, TAFloat, TAInt};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Supertrend indicator over NumPy arrays.
///
/// The Supertrend indicator is a trend-following indicator that combines Average True Range (ATR)
/// with basic upper and lower bands to identify trend direction and potential reversal points.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   high: High prices as a 1-D NumPy array of type `f64`.
///   low: Low prices as a 1-D NumPy array of type `f64`.
///   close: Close prices as a 1-D NumPy array of type `f64`.
///   period: Period for ATR calculation (typically 7-14). Must be positive.
///   multiplier: ATR multiplier (typically 2-4).
///
/// Returns:
///   A tuple of five 1-D NumPy arrays:
///   - trend: Array containing trend direction (1.0 for uptrend, -1.0 for downtrend)
///   - supertrend: Array containing Supertrend values
///   - atr: Array containing ATR values
///   - upper: Array containing upper band values
///   - lower: Array containing lower band values
///   All arrays have the same length as the input, with the first `period-1` elements containing NaN values.
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
///   >>> trend, supertrend, atr, upper, lower = kand.supertrend(high, low, close, 3, 3.0)
///   ```
#[pyfunction]
#[pyo3(name = "supertrend", signature = (high, low, close, period, multiplier))]
pub fn supertrend_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
    period: usize,
    multiplier: TAFloat,
) -> PyResult<(
    Py<PyArray1<TAInt>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    // Convert the input NumPy arrays to Rust slices
    let input_high = high.as_slice()?;
    let input_low = low.as_slice()?;
    let input_close = close.as_slice()?;
    let len = input_high.len();

    // Create new output arrays using vec
    let mut output_trend = vec![0; len];
    let mut output_supertrend = vec![0.0; len];
    let mut output_atr = vec![0.0; len];
    let mut output_upper = vec![0.0; len];
    let mut output_lower = vec![0.0; len];

    // Perform the Supertrend calculation while releasing the GIL
    py.allow_threads(|| {
        supertrend::supertrend(
            input_high,
            input_low,
            input_close,
            period,
            multiplier,
            output_trend.as_mut_slice(),
            output_supertrend.as_mut_slice(),
            output_atr.as_mut_slice(),
            output_upper.as_mut_slice(),
            output_lower.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output arrays to Python objects
    Ok((
        output_trend.into_pyarray(py).into(),
        output_supertrend.into_pyarray(py).into(),
        output_atr.into_pyarray(py).into(),
        output_upper.into_pyarray(py).into(),
        output_lower.into_pyarray(py).into(),
    ))
}

/// Calculates a single Supertrend value incrementally.
///
/// This function provides an optimized way to calculate the latest Supertrend value
/// using previous values, making it ideal for real-time calculations.
///
/// Args:
///   high: Current period's high price
///   low: Current period's low price
///   close: Current period's close price
///   prev_close: Previous period's close price
///   prev_atr: Previous period's ATR value
///   prev_trend: Previous period's trend direction (1 for uptrend, -1 for downtrend)
///   prev_upper: Previous period's upper band
///   prev_lower: Previous period's lower band
///   period: ATR calculation period (typically 7-14)
///   multiplier: ATR multiplier (typically 2-4)
///
/// Returns:
///   A tuple containing:
///   - trend: Current trend direction (1 for uptrend, -1 for downtrend)
///   - supertrend: Current Supertrend value
///   - atr: Current ATR value
///   - upper: Current upper band
///   - lower: Current lower band
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> trend, supertrend, atr, upper, lower = kand.supertrend_incremental(
///   ...     15.0,   # Current high
///   ...     11.0,   # Current low
///   ...     14.0,   # Current close
///   ...     11.0,   # Previous close
///   ...     2.0,    # Previous ATR
///   ...     1,      # Previous trend
///   ...     16.0,   # Previous upper band
///   ...     10.0,   # Previous lower band
///   ...     7,      # ATR period
///   ...     3.0,    # Multiplier
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "supertrend_incremental", signature = (
    high,
    low,
    close,
    prev_close,
    prev_atr,
    prev_trend,
    prev_upper,
    prev_lower,
    period,
    multiplier
))]
pub fn supertrend_incremental_py(
    high: TAFloat,
    low: TAFloat,
    close: TAFloat,
    prev_close: TAFloat,
    prev_atr: TAFloat,
    prev_trend: TAInt,
    prev_upper: TAFloat,
    prev_lower: TAFloat,
    period: usize,
    multiplier: TAFloat,
) -> PyResult<(TAInt, TAFloat, TAFloat, TAFloat, TAFloat)> {
    supertrend::supertrend_incremental(
        high, low, close, prev_close, prev_atr, prev_trend, prev_upper, prev_lower, period,
        multiplier,
    )
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
