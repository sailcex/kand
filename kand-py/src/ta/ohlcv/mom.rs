use kand::{TAFloat, ohlcv::mom};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Momentum (MOM) over a NumPy array.
///
/// Momentum measures the change in price between the current price and the price n periods ago.
///
/// Args:
///   data: Input data as a 1-D NumPy array of type `TAFloat`.
///   period: Window size for momentum calculation. Must be positive and less than input length.
///
/// Returns:
///   A new 1-D NumPy array containing the momentum values. The array has the same length as the input,
///   with the first `period` elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> data = np.array([2.0, 4.0, 6.0, 8.0, 10.0])
///   >>> result = kand.mom(data, 2)
///   >>> print(result)
///   [nan, nan, 4.0, 4.0, 4.0]
///   ```
#[pyfunction]
#[pyo3(name = "mom", signature = (data, period))]
pub fn mom_py(
    py: Python,
    data: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    // Convert the input NumPy array to a Rust slice.
    let input = data.as_slice()?;
    let len = input.len();

    // Create a new output array using vec
    let mut output = vec![0.0; len];

    // Perform the momentum calculation while releasing the GIL to allow other Python threads to run.
    py.allow_threads(|| mom::mom(input, period, output.as_mut_slice()))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output array to a Python object
    Ok(output.into_pyarray(py).into())
}

/// Calculates the next Momentum (MOM) value incrementally.
///
/// This function provides an optimized way to calculate the latest momentum value
/// when streaming data is available, without needing the full price history.
///
/// Args:
///
///   current_price: The current period's price value.
///   old_price: The price value from n periods ago.
///
/// Returns:
///   The calculated momentum value.
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> momentum = kand.mom_incremental(10.0, 6.0)
///   >>> print(momentum)
///   4.0
///   ```
#[pyfunction]
#[pyo3(name = "mom_incremental", signature = (current_price, old_price))]
pub fn mom_incremental_py(
    py: Python,
    current_price: TAFloat,
    old_price: TAFloat,
) -> PyResult<TAFloat> {
    // Perform the incremental momentum calculation while releasing the GIL
    py.allow_threads(|| mom::mom_incremental(current_price, old_price))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
