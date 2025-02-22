use kand::{stats::max, TAFloat};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculate Maximum Value for a NumPy array
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   prices: Input prices as a 1-D NumPy array of type `f64`.
///   period: Period for MAX calculation (must be >= 2).
///
/// Returns:
///   A 1-D NumPy array containing MAX values. The first (period-1) elements contain NaN values.
///
/// Note:
///   This function releases the Python GIL during computation using `py.allow_threads()` to enable
///   concurrent Python execution.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> prices = np.array([1.0, 2.0, 3.0, 2.5, 4.0])
///   >>> max_values = kand.max(prices, 3)
///   ```
#[pyfunction]
#[pyo3(name = "max", signature = (prices, period))]
pub fn max_py(
    py: Python,
    prices: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    // Convert input NumPy array to Rust slice
    let input_prices = prices.as_slice()?;
    let len = input_prices.len();

    // Create output array using vec
    let mut output_max = vec![0.0; len];

    // Perform MAX calculation while releasing the GIL
    py.allow_threads(|| max::max(input_prices, period, &mut output_max))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert output array to Python object
    Ok(output_max.into_pyarray(py).into())
}

/// Calculate the latest Maximum Value incrementally
///
/// Args:
///   py: Python interpreter token
///   price: Current period's price
///   prev_max: Previous period's MAX value
///   old_price: Price being removed from the period
///   period: Period for MAX calculation (must be >= 2)
///
/// Returns:
///   The new MAX value
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> new_max = kand.max_incremental(10.5, 11.0, 9.0, 14)
///   ```
#[pyfunction]
#[pyo3(name = "max_incremental")]
pub fn max_incremental_py(
    py: Python,
    price: TAFloat,
    prev_max: TAFloat,
    old_price: TAFloat,
    period: usize,
) -> PyResult<TAFloat> {
    // Perform incremental MAX calculation while releasing the GIL
    py.allow_threads(|| max::max_incremental(price, prev_max, old_price, period))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
