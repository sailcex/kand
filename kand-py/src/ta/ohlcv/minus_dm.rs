use kand::{TAFloat, ohlcv::minus_dm};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Minus Directional Movement (-DM) over NumPy arrays.
///
/// Minus Directional Movement (-DM) measures downward price movement and is used as part of the
/// Directional Movement System developed by J. Welles Wilder.
///
/// Args:
///   high: Input high prices as a 1-D NumPy array of type `TAFloat`.
///   low: Input low prices as a 1-D NumPy array of type `TAFloat`.
///   period: Window size for -DM calculation. Must be positive and less than input length.
///
/// Returns:
///   A new 1-D NumPy array containing the -DM values. The array has the same length as the input,
///   with the first `period-1` elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([35266.0, 35247.5, 35235.7, 35190.8, 35182.0])
///   >>> low = np.array([35216.1, 35206.5, 35180.0, 35130.7, 35153.6])
///   >>> result = kand.minus_dm(high, low, 3)
///   ```
#[pyfunction]
#[pyo3(name = "minus_dm", signature = (high, low, period))]
pub fn minus_dm_py(
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

    // Perform the -DM calculation while releasing the GIL to allow other Python threads to run
    py.allow_threads(|| minus_dm::minus_dm(input_high, input_low, period, output.as_mut_slice()))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output array to a Python object
    Ok(output.into_pyarray(py).into())
}

/// Calculates the next -DM value incrementally using previous values.
///
/// This function provides an efficient way to update -DM with new price data without recalculating the entire series.
/// It maintains the same mathematical properties as the full calculation.
///
/// Args:
///
///   high: Current high price as `TAFloat`.
///   prev_high: Previous high price as `TAFloat`.
///   low: Current low price as `TAFloat`.
///   prev_low: Previous low price as `TAFloat`.
///   prev_minus_dm: Previous -DM value as `TAFloat`.
///   period: Calculation period (must be between 2 and 100).
///
/// Returns:
///   The next -DM value.
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> next_minus_dm = kand.minus_dm_inc(
///   ...     35182.0,  # high
///   ...     35190.8,  # prev_high
///   ...     35153.6,  # low
///   ...     35130.7,  # prev_low
///   ...     2.5,      # prev_minus_dm
///   ...     14        # period
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "minus_dm_inc", signature = (high, prev_high, low, prev_low, prev_minus_dm, period))]
pub fn minus_dm_inc_py(
    py: Python,
    high: TAFloat,
    prev_high: TAFloat,
    low: TAFloat,
    prev_low: TAFloat,
    prev_minus_dm: TAFloat,
    period: usize,
) -> PyResult<TAFloat> {
    // Perform the incremental -DM calculation while releasing the GIL
    py.allow_threads(|| {
        minus_dm::minus_dm_inc(high, prev_high, low, prev_low, prev_minus_dm, period)
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
