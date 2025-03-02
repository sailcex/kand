use kand::{TAFloat, TAInt, ohlcv::cdl_gravestone_doji};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Detects Gravestone Doji candlestick patterns in price data.
///
/// Args:
///   open: Opening prices as a 1-D NumPy array of type `TAFloat`.
///   high: High prices as a 1-D NumPy array of type `TAFloat`.
///   low: Low prices as a 1-D NumPy array of type `TAFloat`.
///   close: Close prices as a 1-D NumPy array of type `TAFloat`.
///   body_percent: Maximum body size as percentage of total range (typically 5%).
///
/// Returns:
///   A 1-D NumPy array containing pattern signals:
///   - -100: Bearish Gravestone Doji pattern detected
///   - 0: No pattern detected
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> open = np.array([100.0, 101.0, 102.0])
///   >>> high = np.array([102.0, 103.0, 104.0])
///   >>> low = np.array([98.0, 99.0, 100.0])
///   >>> close = np.array([101.0, 102.0, 103.0])
///   >>> signals = kand.cdl_gravestone_doji(open, high, low, close, 5.0)
///   ```
#[pyfunction]
#[pyo3(name = "cdl_gravestone_doji", signature = (open, high, low, close, body_percent))]
pub fn cdl_gravestone_doji_py(
    py: Python,
    open: PyReadonlyArray1<TAFloat>,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
    body_percent: TAFloat,
) -> PyResult<Py<PyArray1<TAInt>>> {
    let input_open = open.as_slice()?;
    let input_high = high.as_slice()?;
    let input_low = low.as_slice()?;
    let input_close = close.as_slice()?;
    let len = input_open.len();

    let mut output_signals = vec![0; len];

    py.allow_threads(|| {
        cdl_gravestone_doji::cdl_gravestone_doji(
            input_open,
            input_high,
            input_low,
            input_close,
            body_percent,
            &mut output_signals,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok(output_signals.into_pyarray(py).into())
}

/// Detects a Gravestone Doji pattern in a single candlestick.
///
/// Args:
///
///   open: Opening price.
///   high: High price.
///   low: Low price.
///   close: Close price.
///   body_percent: Maximum body size as percentage of total range.
///
/// Returns:
///   Signal value:
///   - -100: Bearish Gravestone Doji pattern detected
///   - 0: No pattern detected
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> signal = kand.cdl_gravestone_doji_incremental(100.0, 102.0, 98.0, 100.1, 5.0)
///   ```
#[pyfunction]
#[pyo3(name = "cdl_gravestone_doji_incremental", signature = (open, high, low, close, body_percent))]
pub fn cdl_gravestone_doji_incremental_py(
    py: Python,
    open: TAFloat,
    high: TAFloat,
    low: TAFloat,
    close: TAFloat,
    body_percent: TAFloat,
) -> PyResult<TAInt> {
    py.allow_threads(|| {
        cdl_gravestone_doji::cdl_gravestone_doji_incremental(open, high, low, close, body_percent)
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
