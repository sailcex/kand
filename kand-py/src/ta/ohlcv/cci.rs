use kand::{TAFloat, ohlcv::cci};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Commodity Channel Index (CCI) over NumPy arrays.
///
/// The CCI is a momentum-based oscillator used to help determine when an investment vehicle is reaching
/// a condition of being overbought or oversold.
///
/// Args:
///   high: High prices as a 1-D NumPy array of type `TAFloat`.
///   low: Low prices as a 1-D NumPy array of type `TAFloat`.
///   close: Close prices as a 1-D NumPy array of type `TAFloat`.
///   period: Window size for CCI calculation. Must be positive and less than input length.
///
/// Returns:
///   A tuple of 1-D NumPy arrays containing:
///   - CCI values
///   - Typical prices
///   - SMA of typical prices
///   - Mean deviation values
///   Each array has the same length as the input, with the first `period-1` elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([24.20, 24.07, 24.04, 23.87, 23.67])
///   >>> low = np.array([23.85, 23.72, 23.64, 23.37, 23.46])
///   >>> close = np.array([23.89, 23.95, 23.67, 23.78, 23.50])
///   >>> cci, tp, sma_tp, mean_dev = kand.cci(high, low, close, 3)
///   >>> print(cci)
///   [nan, nan, -100.0, 66.67, -133.33]
///   ```
#[pyfunction]
#[pyo3(name = "cci", signature = (high, low, close, period))]
pub fn cci_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<(
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
    let mut output_cci = vec![0.0; len];
    let mut output_tp = vec![0.0; len];
    let mut output_sma_tp = vec![0.0; len];
    let mut output_mean_dev = vec![0.0; len];

    // Perform the CCI calculation while releasing the GIL to allow other Python threads to run
    py.allow_threads(|| {
        cci::cci(
            input_high,
            input_low,
            input_close,
            period,
            output_cci.as_mut_slice(),
            output_tp.as_mut_slice(),
            output_sma_tp.as_mut_slice(),
            output_mean_dev.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output arrays to Python objects
    Ok((
        output_cci.into_pyarray(py).into(),
        output_tp.into_pyarray(py).into(),
        output_sma_tp.into_pyarray(py).into(),
        output_mean_dev.into_pyarray(py).into(),
    ))
}

/// Calculates the next CCI value incrementally.
///
/// Args:
///
///   prev_sma_tp: Previous SMA value of typical prices.
///   new_high: New high price.
///   new_low: New low price.
///   new_close: New close price.
///   old_high: Old high price to be removed.
///   old_low: Old low price to be removed.
///   old_close: Old close price to be removed.
///   period: Window size for CCI calculation.
///   tp_buffer: List containing the last `period` typical prices.
///
/// Returns:
///   The next CCI value.
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> prev_sma_tp = 100.0
///   >>> new_high = 105.0
///   >>> new_low = 95.0
///   >>> new_close = 100.0
///   >>> old_high = 102.0
///   >>> old_low = 98.0
///   >>> old_close = 100.0
///   >>> period = 14
///   >>> tp_buffer = [100.0] * period
///   >>> next_cci = kand.cci_inc(prev_sma_tp, new_high, new_low, new_close,
///   ...                                  old_high, old_low, old_close, period, tp_buffer)
///   ```
#[pyfunction]
#[pyo3(name = "cci_inc", signature = (prev_sma_tp, new_high, new_low, new_close, old_high, old_low, old_close, period, tp_buffer))]
pub fn cci_inc_py(
    py: Python,
    prev_sma_tp: TAFloat,
    new_high: TAFloat,
    new_low: TAFloat,
    new_close: TAFloat,
    old_high: TAFloat,
    old_low: TAFloat,
    old_close: TAFloat,
    period: usize,
    tp_buffer: Vec<TAFloat>,
) -> PyResult<TAFloat> {
    let mut buffer = tp_buffer;
    py.allow_threads(|| {
        cci::cci_inc(
            prev_sma_tp,
            new_high,
            new_low,
            new_close,
            old_high,
            old_low,
            old_close,
            period,
            &mut buffer,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
