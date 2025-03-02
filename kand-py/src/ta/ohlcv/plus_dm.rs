use kand::{TAFloat, ohlcv::plus_dm};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Plus Directional Movement (+DM) over NumPy arrays.
///
/// Plus Directional Movement (+DM) measures upward price movement and is used as part of the
/// Directional Movement System developed by J. Welles Wilder.
///
/// Args:
///   high: Input high prices as a 1-D NumPy array of type `TAFloat`.
///   low: Input low prices as a 1-D NumPy array of type `TAFloat`.
///   period: Window size for +DM calculation. Must be positive and less than input length.
///
/// Returns:
///   A new 1-D NumPy array containing the +DM values. The array has the same length as the input,
///   with the first `period-1` elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([35266.0, 35247.5, 35235.7, 35190.8, 35182.0])
///   >>> low = np.array([35216.1, 35206.5, 35180.0, 35130.7, 35153.6])
///   >>> result = kand.plus_dm(high, low, 3)
///   ```
#[pyfunction]
#[pyo3(name = "plus_dm", signature = (high, low, period))]
pub fn plus_dm_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    // Convert the input NumPy arrays to Rust slices
    let input_high = high.as_slice()?;
    let input_low = low.as_slice()?;
    let len = input_high.len();

    // Create a new output array using vec
    let mut output = vec![0.0; len];

    // Perform the Plus DM calculation while releasing the GIL to allow other Python threads to run
    py.allow_threads(|| plus_dm::plus_dm(input_high, input_low, period, output.as_mut_slice()))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output array to a Python object
    Ok(output.into_pyarray(py).into())
}

/// Calculates the next Plus DM value incrementally using previous values.
///
/// This function enables real-time calculation of Plus DM by using the previous Plus DM value
/// and current price data, avoiding the need to recalculate the entire series.
///
/// Args:
///   high: Current high price as `TAFloat`.
///   prev_high: Previous high price as `TAFloat`.
///   low: Current low price as `TAFloat`.
///   prev_low: Previous low price as `TAFloat`.
///   prev_plus_dm: Previous Plus DM value as `TAFloat`.
///   period: Smoothing period (>= 2).
///
/// Returns:
///   The latest Plus DM value.
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> new_plus_dm = kand.plus_dm_incremental(
///   ...     10.5,  # high
///   ...     10.0,  # prev_high
///   ...     9.8,   # low
///   ...     9.5,   # prev_low
///   ...     0.45,  # prev_plus_dm
///   ...     14     # period
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "plus_dm_incremental", signature = (high, prev_high, low, prev_low, prev_plus_dm, period))]
pub fn plus_dm_incremental_py(
    high: TAFloat,
    prev_high: TAFloat,
    low: TAFloat,
    prev_low: TAFloat,
    prev_plus_dm: TAFloat,
    period: usize,
) -> PyResult<TAFloat> {
    plus_dm::plus_dm_incremental(high, prev_high, low, prev_low, prev_plus_dm, period)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
