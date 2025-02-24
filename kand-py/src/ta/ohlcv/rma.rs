use kand::{TAFloat, ohlcv::rma};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Running Moving Average (RMA) over a NumPy array.
///
/// The Running Moving Average is similar to an Exponential Moving Average (EMA) but uses a different
/// smoothing factor. It is calculated using a weighted sum of the current value and previous RMA value,
/// with weights determined by the period size.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   data: Input data as a 1-D NumPy array of type `f64`.
///   period: Window size for RMA calculation. Must be positive and less than input length.
///
/// Returns:
///   A new 1-D NumPy array containing the RMA values. The array has the same length as the input,
///   with the first `period-1` elements containing NaN values.
///
/// Note:
///   This function releases the Python GIL during computation using `py.allow_threads()` to enable
///   concurrent Python execution.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> data = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
///   >>> result = kand.rma(data, 3)
///   >>> print(result)
///   [nan, nan, 2.0, 2.67, 3.44]
///   ```
#[pyfunction]
#[pyo3(name = "rma", signature = (data, period))]
pub fn rma_py(
    py: Python,
    data: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    // Convert the input NumPy array to a Rust slice.
    let input = data.as_slice()?;
    let len = input.len();

    // Create a new output array using vec
    let mut output = vec![0.0; len];

    // Perform the RMA calculation while releasing the GIL to allow other Python threads to run.
    py.allow_threads(|| rma::rma(input, period, output.as_mut_slice()))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output array to a Python object
    Ok(output.into_pyarray(py).into())
}

/// Calculates the next RMA value incrementally.
///
/// This function provides an optimized way to calculate the latest RMA value
/// when streaming data is available, without needing the full price history.
///
/// Args:
///   current_price: The current period's price value.
///   prev_rma: The previous period's RMA value.
///   period: The smoothing period (must be >= 2).
///
/// Returns:
///   The calculated RMA value.
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> new_rma = kand.rma_incremental(10.0, 9.5, 14)
///   ```
#[pyfunction]
#[pyo3(name = "rma_incremental", signature = (current_price, prev_rma, period))]
pub fn rma_incremental_py(
    current_price: TAFloat,
    prev_rma: TAFloat,
    period: usize,
) -> PyResult<TAFloat> {
    rma::rma_incremental(current_price, prev_rma, period)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
