use kand::{TAFloat, TAInt, ohlcv::cdl_inverted_hammer};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Detects Inverted Hammer candlestick patterns in price data.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   open: Opening prices as a 1-D NumPy array of type `f32`.
///   high: High prices as a 1-D NumPy array of type `f32`.
///   low: Low prices as a 1-D NumPy array of type `f32`.
///   close: Close prices as a 1-D NumPy array of type `f32`.
///   period: Period for EMA calculation of body sizes.
///   factor: Minimum ratio of upper shadow to body length.
///
/// Returns:
///   A tuple of two 1-D NumPy arrays containing:
///   - Pattern signals:
///     - 100: Bullish Inverted Hammer pattern detected
///     - 0: No pattern detected
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
///   >>> signals, body_avg = kand.cdl_inverted_hammer(open, high, low, close, 14, 2.0)
///   ```
#[pyfunction]
#[pyo3(name = "cdl_inverted_hammer", signature = (open, high, low, close, period, factor))]
pub fn cdl_inverted_hammer_py(
    py: Python,
    open: PyReadonlyArray1<TAFloat>,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
    period: usize,
    factor: TAFloat,
) -> PyResult<(Py<PyArray1<TAInt>>, Py<PyArray1<TAFloat>>)> {
    let input_open = open.as_slice()?;
    let input_high = high.as_slice()?;
    let input_low = low.as_slice()?;
    let input_close = close.as_slice()?;
    let len = input_open.len();

    let mut output_signals = vec![0; len];
    let mut output_body_avg = vec![0.0; len];

    py.allow_threads(|| {
        cdl_inverted_hammer::cdl_inverted_hammer(
            input_open,
            input_high,
            input_low,
            input_close,
            period,
            factor,
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

/// Detects an Inverted Hammer pattern in a single candlestick.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   open: Opening price.
///   high: High price.
///   low: Low price.
///   close: Close price.
///   prev_body_avg: Previous EMA value of body sizes.
///   period: Period for EMA calculation.
///   factor: Minimum ratio of upper shadow to body length.
///
/// Returns:
///   A tuple containing:
///   - Signal value:
///     - 100: Bullish Inverted Hammer pattern detected
///     - 0: No pattern detected
///   - Updated EMA value of body sizes
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> signal, body_avg = kand.cdl_inverted_hammer_incremental(100.0, 102.0, 98.0, 100.1, 0.5, 14, 2.0)
///   ```
#[pyfunction]
#[pyo3(name = "cdl_inverted_hammer_incremental", signature = (open, high, low, close, prev_body_avg, period, factor))]
pub fn cdl_inverted_hammer_incremental_py(
    py: Python,
    open: TAFloat,
    high: TAFloat,
    low: TAFloat,
    close: TAFloat,
    prev_body_avg: TAFloat,
    period: usize,
    factor: TAFloat,
) -> PyResult<(TAInt, TAFloat)> {
    py.allow_threads(|| {
        cdl_inverted_hammer::cdl_inverted_hammer_incremental(
            open,
            high,
            low,
            close,
            prev_body_avg,
            period,
            factor,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
