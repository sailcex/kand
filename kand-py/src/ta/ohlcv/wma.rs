use kand::{TAFloat, ohlcv::wma};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Weighted Moving Average (WMA) over a NumPy array.
///
/// The Weighted Moving Average assigns linearly decreasing weights to each price in the period,
/// giving more importance to recent prices and less to older ones.
///
/// Args:
///     py: Python interpreter token required for GIL management.
///     data: Input data as a 1-D NumPy array of type `f32`.
///     period: Window size for WMA calculation. Must be >= 2.
///
/// Returns:
///     A new 1-D NumPy array containing the WMA values. The array has the same length as the input,
///     with the first `period-1` elements containing NaN values.
///
/// Note:
///     This function releases the Python GIL during computation using `py.allow_threads()` to enable
///     concurrent Python execution.
///
/// Examples:
///     ```python
///     >>> import numpy as np
///     >>> import kand
///     >>> data = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
///     >>> result = kand.wma(data, 3)
///     >>> print(result)
///     [nan, nan, 2.0, 3.0, 4.0]
///     ```
#[pyfunction]
#[pyo3(name = "wma", signature = (data, period))]
pub fn wma_py(
    py: Python,
    data: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    let input = data.as_slice()?;
    let len = input.len();
    let mut output = vec![0.0; len];

    py.allow_threads(|| wma::wma(input, period, output.as_mut_slice()))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok(output.into_pyarray(py).into())
}

/// Incrementally calculates the next WMA value.
///
/// This function provides an optimized way to calculate the latest WMA value
/// by using a window of the most recent prices.
///
/// Args:
///     input_window: Array of price values ordered from newest to oldest.
///     period: The time period for WMA calculation (must be >= 2).
///
/// Returns:
///     The next WMA value.
///
/// Examples:
///     ```python
///     >>> import kand
///     >>> window = [5.0, 4.0, 3.0]  # newest to oldest
///     >>> wma = kand.wma_incremental(window, 3)
///     >>> print(wma)
///     4.333333333333333
///     ```
#[pyfunction]
#[pyo3(name = "wma_incremental", signature = (input_window, period))]
pub fn wma_incremental_py(input_window: Vec<TAFloat>, period: usize) -> PyResult<TAFloat> {
    wma::wma_incremental(&input_window, period)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
