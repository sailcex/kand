use kand::{TAFloat, ohlcv::minus_di};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Minus Directional Indicator (-DI) over NumPy arrays.
///
/// The -DI measures the presence and strength of a downward price trend. It is one component used in calculating
/// the Average Directional Index (ADX), which helps determine trend strength.
///
/// Args:
///   high: High prices as a 1-D NumPy array of type `TAFloat`.
///   low: Low prices as a 1-D NumPy array of type `TAFloat`.
///   close: Close prices as a 1-D NumPy array of type `TAFloat`.
///   period: Window size for -DI calculation. Must be positive and less than input length.
///
/// Returns:
///   A tuple of three 1-D NumPy arrays containing:
///   - The -DI values
///   - The smoothed -DM values
///   - The smoothed TR values
///   Each array has the same length as the input, with the first `period` elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([35.0, 36.0, 35.5, 35.8, 36.2])
///   >>> low = np.array([34.0, 35.0, 34.5, 34.8, 35.2])
///   >>> close = np.array([34.5, 35.5, 35.0, 35.3, 35.7])
///   >>> minus_di, smoothed_minus_dm, smoothed_tr = kand.minus_di(high, low, close, 3)
///   >>> print(minus_di)
///   [nan, nan, nan, 25.3, 24.1]
///   ```
#[pyfunction]
#[pyo3(name = "minus_di", signature = (high, low, close, period))]
pub fn minus_di_py(
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
    let mut output_minus_di = vec![0.0; len];
    let mut output_smoothed_minus_dm = vec![0.0; len];
    let mut output_smoothed_tr = vec![0.0; len];

    // Perform the -DI calculation while releasing the GIL to allow other Python threads to run
    py.allow_threads(|| {
        minus_di::minus_di(
            input_high,
            input_low,
            input_close,
            period,
            output_minus_di.as_mut_slice(),
            output_smoothed_minus_dm.as_mut_slice(),
            output_smoothed_tr.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output arrays to Python objects
    Ok((
        output_minus_di.into_pyarray(py).into(),
        output_smoothed_minus_dm.into_pyarray(py).into(),
        output_smoothed_tr.into_pyarray(py).into(),
    ))
}

/// Calculates the next -DI value incrementally using previous smoothed values.
///
/// This function provides an efficient way to update -DI with new price data without recalculating the entire series.
/// It maintains the same mathematical properties as the full calculation.
///
/// Args:
///
///   high: Current high price as `TAFloat`.
///   low: Current low price as `TAFloat`.
///   prev_high: Previous high price as `TAFloat`.
///   prev_low: Previous low price as `TAFloat`.
///   prev_close: Previous close price as `TAFloat`.
///   prev_smoothed_minus_dm: Previous smoothed -DM value as `TAFloat`.
///   prev_smoothed_tr: Previous smoothed TR value as `TAFloat`.
///   period: Calculation period (>= 2).
///
/// Returns:
///   A tuple of three values:
///   - The new -DI value
///   - The new smoothed -DM value
///   - The new smoothed TR value
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> minus_di, smoothed_minus_dm, smoothed_tr = kand.minus_di_inc(
///   ...     36.2,  # high
///   ...     35.2,  # low
///   ...     35.8,  # prev_high
///   ...     34.8,  # prev_low
///   ...     35.3,  # prev_close
///   ...     0.5,   # prev_smoothed_minus_dm
///   ...     1.5,   # prev_smoothed_tr
///   ...     14     # period
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "minus_di_inc", signature = (high, low, prev_high, prev_low, prev_close, prev_smoothed_minus_dm, prev_smoothed_tr, period))]
pub fn minus_di_inc_py(
    py: Python,
    high: TAFloat,
    low: TAFloat,
    prev_high: TAFloat,
    prev_low: TAFloat,
    prev_close: TAFloat,
    prev_smoothed_minus_dm: TAFloat,
    prev_smoothed_tr: TAFloat,
    period: usize,
) -> PyResult<(TAFloat, TAFloat, TAFloat)> {
    // Perform the incremental -DI calculation while releasing the GIL
    py.allow_threads(|| {
        minus_di::minus_di_inc(
            high,
            low,
            prev_high,
            prev_low,
            prev_close,
            prev_smoothed_minus_dm,
            prev_smoothed_tr,
            period,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
