use kand::{TAFloat, ohlcv::rocr100};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Rate of Change Ratio * 100 (ROCR100) over a NumPy array.
///
/// ROCR100 is a momentum indicator that measures the percentage change in price over a specified period.
/// It compares the current price to a past price and expresses the ratio as a percentage.
/// Values above 100 indicate price increases, while values below 100 indicate price decreases.
///
/// Args:
///   data: Input price data as a 1-D NumPy array of type `TAFloat`.
///   period: Number of periods to look back. Must be >= 2.
///
/// Returns:
///   A new 1-D NumPy array containing the ROCR100 values. The array has the same length as the input,
///   with the first `period` elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> data = np.array([10.0, 10.5, 11.2, 10.8, 11.5])
///   >>> result = kand.rocr100(data, 2)
///   >>> print(result)
///   [nan, nan, 106.67, 102.86, 106.48]
///   ```
#[pyfunction]
#[pyo3(name = "rocr100", signature = (data, period))]
pub fn rocr100_py(
    py: Python,
    data: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    // Convert the input NumPy array to a Rust slice
    let input = data.as_slice()?;
    let len = input.len();

    // Create a new output array using vec
    let mut output = vec![0.0; len];

    // Perform the ROCR100 calculation while releasing the GIL to allow other Python threads to run
    py.allow_threads(|| rocr100::rocr100(input, period, output.as_mut_slice()))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output array to a Python object
    Ok(output.into_pyarray(py).into())
}

/// Calculates a single ROCR100 value incrementally.
///
/// This function provides an optimized way to calculate the latest ROCR100 value
/// when streaming data is available, without needing the full price history.
///
/// Args:
///   current_price: The current period's price value.
///   prev_price: The price from n periods ago.
///
/// Returns:
///   The calculated ROCR100 value.
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> rocr100 = kand.rocr100_incremental(11.5, 10.0)
///   >>> print(rocr100)
///   115.0
///   ```
#[pyfunction]
#[pyo3(name = "rocr100_incremental", signature = (current_price, prev_price))]
pub fn rocr100_incremental_py(current_price: TAFloat, prev_price: TAFloat) -> PyResult<TAFloat> {
    rocr100::rocr100_incremental(current_price, prev_price)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
