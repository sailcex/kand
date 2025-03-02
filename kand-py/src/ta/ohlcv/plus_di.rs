use kand::{TAFloat, ohlcv::plus_di};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Plus Directional Indicator (+DI) over NumPy arrays.
///
/// +DI measures the presence and strength of an upward price trend. It is one component used in calculating
/// the Average Directional Index (ADX), which helps determine trend strength.
///
/// Args:
///   high: High prices as a 1-D NumPy array of type `TAFloat`.
///   low: Low prices as a 1-D NumPy array of type `TAFloat`.
///   close: Close prices as a 1-D NumPy array of type `TAFloat`.
///   period: Window size for +DI calculation. Must be positive and less than input length.
///
/// Returns:
///   A tuple of three 1-D NumPy arrays containing:
///   - +DI values
///   - Smoothed +DM values
///   - Smoothed TR values
///   Each array has the same length as the input, with the first `period` elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([10.0, 12.0, 11.5, 11.0])
///   >>> low = np.array([9.0, 10.0, 10.0, 9.5])
///   >>> close = np.array([9.5, 11.0, 10.5, 10.0])
///   >>> plus_di, smoothed_plus_dm, smoothed_tr = kand.plus_di(high, low, close, 2)
///   ```
#[pyfunction]
#[pyo3(name = "plus_di", signature = (high, low, close, period))]
pub fn plus_di_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<(
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
    let mut output_plus_di = vec![0.0; len];
    let mut output_smoothed_plus_dm = vec![0.0; len];
    let mut output_smoothed_tr = vec![0.0; len];

    // Perform the +DI calculation while releasing the GIL to allow other Python threads to run
    py.allow_threads(|| {
        plus_di::plus_di(
            input_high,
            input_low,
            input_close,
            period,
            output_plus_di.as_mut_slice(),
            output_smoothed_plus_dm.as_mut_slice(),
            output_smoothed_tr.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output arrays to Python objects
    Ok((
        output_plus_di.into_pyarray(py).into(),
        output_smoothed_plus_dm.into_pyarray(py).into(),
        output_smoothed_tr.into_pyarray(py).into(),
    ))
}

/// Calculates the next +DI value incrementally using previous smoothed values.
///
/// This function enables real-time calculation of +DI by using the previous smoothed values
/// and current price data, avoiding the need to recalculate the entire series.
///
/// Args:
///   high: Current high price as `TAFloat`.
///   low: Current low price as `TAFloat`.
///   prev_high: Previous high price as `TAFloat`.
///   prev_low: Previous low price as `TAFloat`.
///   prev_close: Previous close price as `TAFloat`.
///   prev_smoothed_plus_dm: Previous smoothed +DM value as `TAFloat`.
///   prev_smoothed_tr: Previous smoothed TR value as `TAFloat`.
///   period: Smoothing period (>= 2).
///
/// Returns:
///   A tuple containing (latest +DI, new smoothed +DM, new smoothed TR).
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> plus_di, smoothed_plus_dm, smoothed_tr = kand.plus_di_incremental(
///   ...     10.5,  # high
///   ...     9.5,   # low
///   ...     10.0,  # prev_high
///   ...     9.0,   # prev_low
///   ...     9.5,   # prev_close
///   ...     15.0,  # prev_smoothed_plus_dm
///   ...     20.0,  # prev_smoothed_tr
///   ...     14     # period
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "plus_di_incremental", signature = (high, low, prev_high, prev_low, prev_close, prev_smoothed_plus_dm, prev_smoothed_tr, period))]
pub fn plus_di_incremental_py(
    high: TAFloat,
    low: TAFloat,
    prev_high: TAFloat,
    prev_low: TAFloat,
    prev_close: TAFloat,
    prev_smoothed_plus_dm: TAFloat,
    prev_smoothed_tr: TAFloat,
    period: usize,
) -> PyResult<(TAFloat, TAFloat, TAFloat)> {
    plus_di::plus_di_incremental(
        high,
        low,
        prev_high,
        prev_low,
        prev_close,
        prev_smoothed_plus_dm,
        prev_smoothed_tr,
        period,
    )
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
