use kand::{TAFloat, TAInt, ohlcv::cdl_doji};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Detects Doji candlestick patterns in price data.
///
/// Args:
///   open: Opening prices as a 1-D NumPy array of type `TAFloat`.
///   high: High prices as a 1-D NumPy array of type `TAFloat`.
///   low: Low prices as a 1-D NumPy array of type `TAFloat`.
///   close: Close prices as a 1-D NumPy array of type `TAFloat`.
///   body_percent: Maximum body size as percentage of range (e.g. 5.0 for 5%).
///   shadow_equal_percent: Maximum shadow length difference percentage (e.g. 100.0).
///
/// Returns:
///   A 1-D NumPy array containing pattern signals (1.0 = pattern, 0.0 = no pattern).
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> open = np.array([10.0, 10.5, 10.2])
///   >>> high = np.array([11.0, 11.2, 10.8])
///   >>> low = np.array([9.8, 10.1, 9.9])
///   >>> close = np.array([10.3, 10.4, 10.25])
///   >>> signals = kand.cdl_doji(open, high, low, close, 5.0, 100.0)
///   ```
#[pyfunction]
#[pyo3(name = "cdl_doji", signature = (open, high, low, close, body_percent, shadow_equal_percent))]
pub fn cdl_doji_py(
    py: Python,
    open: PyReadonlyArray1<TAFloat>,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
    body_percent: TAFloat,
    shadow_equal_percent: TAFloat,
) -> PyResult<Py<PyArray1<TAInt>>> {
    let input_open = open.as_slice()?;
    let input_high = high.as_slice()?;
    let input_low = low.as_slice()?;
    let input_close = close.as_slice()?;
    let len = input_open.len();

    let mut output_signals = vec![0; len];

    py.allow_threads(|| {
        cdl_doji::cdl_doji(
            input_open,
            input_high,
            input_low,
            input_close,
            body_percent,
            shadow_equal_percent,
            &mut output_signals,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok(output_signals.into_pyarray(py).into())
}

/// Detects a Doji pattern in a single candlestick.
///
/// Args:
///
///   open: Opening price.
///   high: High price.
///   low: Low price.
///   close: Close price.
///   body_percent: Maximum body size as percentage of range.
///   shadow_equal_percent: Maximum shadow length difference percentage.
///
/// Returns:
///   Signal value (1.0 for Doji pattern, 0.0 for no pattern).
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> signal = kand.cdl_doji_inc(10.0, 11.0, 9.8, 10.3, 5.0, 100.0)
///   ```
#[pyfunction]
#[pyo3(name = "cdl_doji_inc", signature = (open, high, low, close, body_percent, shadow_equal_percent))]
pub fn cdl_doji_inc_py(
    py: Python,
    open: TAFloat,
    high: TAFloat,
    low: TAFloat,
    close: TAFloat,
    body_percent: TAFloat,
    shadow_equal_percent: TAFloat,
) -> PyResult<TAInt> {
    py.allow_threads(|| {
        cdl_doji::cdl_doji_inc(open, high, low, close, body_percent, shadow_equal_percent)
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
