use kand::{TAFloat, ohlcv::midpoint};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculates Midpoint values for a NumPy array.
///
/// The Midpoint is a technical indicator that represents the arithmetic mean of the highest and lowest
/// prices over a specified period.
///
/// Args:
///   data: Input price data as a 1-D NumPy array of type `TAFloat`.
///   period: Time period for calculation (must be >= 2).
///
/// Returns:
///   A tuple of three 1-D NumPy arrays containing:
///   - Midpoint values
///   - Highest values for each period
///   - Lowest values for each period
///   Each array has the same length as the input, with initial elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> data = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
///   >>> midpoint, highest, lowest = kand.midpoint(data, 3)
///   ```
#[pyfunction]
#[pyo3(name = "midpoint", signature = (data, period))]
pub fn midpoint_py(
    py: Python,
    data: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    let input = data.as_slice()?;
    let len = input.len();

    let mut output_midpoint = vec![0.0; len];
    let mut output_highest = vec![0.0; len];
    let mut output_lowest = vec![0.0; len];

    py.allow_threads(|| {
        midpoint::midpoint(
            input,
            period,
            output_midpoint.as_mut_slice(),
            output_highest.as_mut_slice(),
            output_lowest.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output_midpoint.into_pyarray(py).into(),
        output_highest.into_pyarray(py).into(),
        output_lowest.into_pyarray(py).into(),
    ))
}

/// Calculates the next Midpoint value incrementally.
///
/// Provides an optimized way to calculate the next Midpoint value when new data arrives,
/// without recalculating the entire series.
///
/// Args:
///   price: Current price value as `TAFloat`.
///   prev_highest: Previous highest value as `TAFloat`.
///   prev_lowest: Previous lowest value as `TAFloat`.
///   period: Time period for calculation (must be >= 2).
///
/// Returns:
///   A tuple containing:
///   - Midpoint value
///   - New highest value
///   - New lowest value
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> midpoint, new_highest, new_lowest = kand.midpoint_incremental(
///   ...     15.0,  # current price
///   ...     16.0,  # previous highest
///   ...     14.0,  # previous lowest
///   ...     14     # period
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "midpoint_incremental", signature = (price, prev_highest, prev_lowest, period))]
pub fn midpoint_incremental_py(
    py: Python,
    price: TAFloat,
    prev_highest: TAFloat,
    prev_lowest: TAFloat,
    period: usize,
) -> PyResult<(TAFloat, TAFloat, TAFloat)> {
    py.allow_threads(|| midpoint::midpoint_incremental(price, prev_highest, prev_lowest, period))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
