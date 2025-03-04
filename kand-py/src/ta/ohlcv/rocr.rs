use kand::{TAFloat, ohlcv::rocr};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Rate of Change Ratio (ROCR) over a NumPy array.
///
/// The Rate of Change Ratio (ROCR) is a momentum indicator that measures the ratio between
/// the current price and the price n periods ago.
///
/// Args:
///   data: Input price data as a 1-D NumPy array of type `TAFloat`.
///   period: Number of periods to look back. Must be >= 2.
///
/// Returns:
///   A new 1-D NumPy array containing the ROCR values. The array has the same length as the input,
///   with the first `period` elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> data = np.array([10.0, 10.5, 11.2, 10.8, 11.5])
///   >>> result = kand.rocr(data, 2)
///   >>> print(result)
///   [nan, nan, 1.12, 1.0286, 1.0648]
///   ```
#[pyfunction]
#[pyo3(name = "rocr", signature = (data, period))]
pub fn rocr_py(
    py: Python,
    data: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    // Convert the input NumPy array to a Rust slice
    let input = data.as_slice()?;
    let len = input.len();

    // Create a new output array using vec
    let mut output = vec![0.0; len];

    // Perform the ROCR calculation while releasing the GIL to allow other Python threads to run
    py.allow_threads(|| rocr::rocr(input, period, output.as_mut_slice()))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output array to a Python object
    Ok(output.into_pyarray(py).into())
}

/// Calculates a single ROCR value incrementally.
///
/// This function provides an optimized way to calculate the latest ROCR value
/// when streaming data is available, without needing the full price history.
///
/// Args:
///   current_price: The current period's price value.
///   prev_price: The price from n periods ago.
///
/// Returns:
///   The calculated ROCR value.
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> rocr = kand.rocr_inc(11.5, 10.0)
///   >>> print(rocr)
///   1.15
///   ```
#[pyfunction]
#[pyo3(name = "rocr_inc", signature = (current_price, prev_price))]
pub fn rocr_inc_py(current_price: TAFloat, prev_price: TAFloat) -> PyResult<TAFloat> {
    rocr::rocr_inc(current_price, prev_price)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
