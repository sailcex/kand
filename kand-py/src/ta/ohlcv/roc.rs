use kand::{TAFloat, ohlcv::roc};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Rate of Change (ROC) over a NumPy array.
///
/// The Rate of Change (ROC) is a momentum oscillator that measures the percentage change in price
/// between the current price and the price n periods ago.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   data: Input price data as a 1-D NumPy array of type `f64`.
///   period: Number of periods to look back. Must be positive.
///
/// Returns:
///   A new 1-D NumPy array containing the ROC values. The array has the same length as the input,
///   with the first `period` elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> data = np.array([10.0, 10.5, 11.2, 10.8, 11.5])
///   >>> result = kand.roc(data, 2)
///   >>> print(result)
///   [nan, nan, 12.0, 2.86, 6.48]
///   ```
#[pyfunction]
#[pyo3(name = "roc", signature = (data, period))]
pub fn roc_py(
    py: Python,
    data: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    // Convert the input NumPy array to a Rust slice
    let input = data.as_slice()?;
    let len = input.len();

    // Create a new output array using vec
    let mut output = vec![0.0; len];

    // Perform the ROC calculation while releasing the GIL to allow other Python threads to run
    py.allow_threads(|| roc::roc(input, period, output.as_mut_slice()))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output array to a Python object
    Ok(output.into_pyarray(py).into())
}

/// Calculates a single ROC value incrementally.
///
/// This function provides an optimized way to calculate the latest ROC value
/// when streaming data is available, without needing the full price history.
///
/// Args:
///   current_price: The current period's price value.
///   prev_price: The price from n periods ago.
///
/// Returns:
///   The calculated ROC value.
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> roc = kand.roc_incremental(11.5, 10.0)
///   >>> print(roc)
///   15.0
///   ```
#[pyfunction]
#[pyo3(name = "roc_incremental", signature = (current_price, prev_price))]
pub fn roc_incremental_py(current_price: TAFloat, prev_price: TAFloat) -> PyResult<TAFloat> {
    roc::roc_incremental(current_price, prev_price)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
