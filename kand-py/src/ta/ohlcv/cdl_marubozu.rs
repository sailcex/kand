use kand::{TAFloat, TAInt, ohlcv::cdl_marubozu};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Detects Marubozu candlestick patterns in price data.
///
/// Args:
///   open: Opening prices as a 1-D NumPy array of type `TAFloat`.
///   high: High prices as a 1-D NumPy array of type `TAFloat`.
///   low: Low prices as a 1-D NumPy array of type `TAFloat`.
///   close: Close prices as a 1-D NumPy array of type `TAFloat`.
///   period: Period for EMA calculation of body sizes.
///   shadow_percent: Maximum shadow size as percentage of body.
///
/// Returns:
///   A tuple of two 1-D NumPy arrays containing:
///   - Pattern signals:
///     - 1.0: Bullish Marubozu pattern detected
///     - -1.0: Bearish Marubozu pattern detected
///     - 0.0: No pattern detected
///   - EMA values of candle body sizes
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> open = np.array([100.0, 101.0, 102.0])
///   >>> high = np.array([102.0, 103.0, 104.0])
///   >>> low = np.array([98.0, 99.0, 100.0])
///   >>> close = np.array([101.0, 102.0, 103.0])
///   >>> signals, body_avg = kand.cdl_marubozu(open, high, low, close, 14, 5.0)
///   ```
#[pyfunction]
#[pyo3(name = "cdl_marubozu", signature = (open, high, low, close, period, shadow_percent))]
pub fn cdl_marubozu_py(
    py: Python,
    open: PyReadonlyArray1<TAFloat>,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
    period: usize,
    shadow_percent: TAFloat,
) -> PyResult<(Py<PyArray1<TAInt>>, Py<PyArray1<TAFloat>>)> {
    let input_open = open.as_slice()?;
    let input_high = high.as_slice()?;
    let input_low = low.as_slice()?;
    let input_close = close.as_slice()?;
    let len = input_open.len();

    let mut output_signals = vec![0; len];
    let mut output_body_avg = vec![0.0; len];

    py.allow_threads(|| {
        cdl_marubozu::cdl_marubozu(
            input_open,
            input_high,
            input_low,
            input_close,
            period,
            shadow_percent,
            &mut output_signals,
            &mut output_body_avg,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output_signals.into_pyarray(py).into(),
        output_body_avg.into_pyarray(py).into(),
    ))
}

/// Detects a Marubozu pattern in a single candlestick.
///
/// Args:
///
///   open: Opening price.
///   high: High price.
///   low: Low price.
///   close: Close price.
///   prev_body_avg: Previous EMA value of body sizes.
///   period: Period for EMA calculation.
///   shadow_percent: Maximum shadow size as percentage of body.
///
/// Returns:
///   A tuple containing:
///   - Signal value:
///     - 1.0: Bullish Marubozu pattern detected
///     - -1.0: Bearish Marubozu pattern detected
///     - 0.0: No pattern detected
///   - Updated EMA value of body sizes
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> signal, body_avg = kand.cdl_marubozu_inc(100.0, 102.0, 98.0, 100.1, 0.5, 14, 5.0)
///   ```
#[pyfunction]
#[pyo3(name = "cdl_marubozu_inc", signature = (open, high, low, close, prev_body_avg, period, shadow_percent))]
pub fn cdl_marubozu_inc_py(
    py: Python,
    open: TAFloat,
    high: TAFloat,
    low: TAFloat,
    close: TAFloat,
    prev_body_avg: TAFloat,
    period: usize,
    shadow_percent: TAFloat,
) -> PyResult<(TAInt, TAFloat)> {
    py.allow_threads(|| {
        cdl_marubozu::cdl_marubozu_inc(
            open,
            high,
            low,
            close,
            prev_body_avg,
            period,
            shadow_percent,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
