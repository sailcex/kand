use kand::{ohlcv::sma, TAFloat};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Simple Moving Average (SMA) over a NumPy array.
///
/// The Simple Moving Average is calculated by taking the arithmetic mean of a window of values
/// that moves across the input array. For each position, it sums the previous `period` values
/// and divides by the period size.
///
/// Args:
///     py: Python interpreter token required for GIL management.
///     data: Input data as a 1-D NumPy array of type `f64`.
///     period: Window size for SMA calculation. Must be positive and less than input length.
///
/// Returns:
///     A new 1-D NumPy array containing the SMA values. The array has the same length as the input,
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
///     >>> result = kand.sma(data, 3)
///     >>> print(result)
///     [nan, nan, 2.0, 3.0, 4.0]
///     ```
#[pyfunction]
#[pyo3(name = "sma", signature = (data, period))]
pub fn sma_py(
    py: Python,
    data: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    // Convert the input NumPy array to a Rust slice.
    let input = data.as_slice()?;
    let len = input.len();

    // Create a new output array using vec
    let mut output = vec![0.0; len];

    // Perform the SMA calculation while releasing the GIL to allow other Python threads to run.
    py.allow_threads(|| sma::sma(input, period, output.as_mut_slice()))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output array to a Python object
    Ok(output.into_pyarray(py).into())
}

/// Incrementally calculates the next SMA value.
///
/// This function provides an optimized way to update an existing SMA value
/// when new data arrives, without recalculating the entire series.
///
/// Args:
///     prev_sma: Previous SMA value.
///     new_price: New price to include in calculation.
///     old_price: Oldest price to remove from calculation.
///     period: The time period for SMA calculation (must be >= 2).
///
/// Returns:
///     The next SMA value.
///
/// Examples:
///     ```python
///     >>> import kand
///     >>> prev_sma = 4.0
///     >>> new_price = 10.0
///     >>> old_price = 2.0
///     >>> period = 3
///     >>> next_sma = kand.sma_incremental(prev_sma, new_price, old_price, period)
///     >>> print(next_sma)
///     6.666666666666666
///     ```
#[pyfunction]
#[pyo3(name = "sma_incremental", signature = (prev_sma, new_price, old_price, period))]
pub fn sma_incremental_py(
    prev_sma: TAFloat,
    new_price: TAFloat,
    old_price: TAFloat,
    period: usize,
) -> PyResult<TAFloat> {
    sma::sma_incremental(prev_sma, new_price, old_price, period)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
