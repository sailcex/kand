use kand::{TAFloat, stats::min};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculate Minimum Value (MIN) for a NumPy array
///
/// The MIN indicator finds the lowest price value within a given time period.
///
/// Args:
///   prices: Input prices as a 1-D NumPy array of type `TAFloat`.
///   period: Period for MIN calculation (must be >= 2).
///
/// Returns:
///   A 1-D NumPy array containing MIN values. First (period-1) elements contain NaN.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> prices = np.array([10.0, 8.0, 6.0, 7.0, 9.0])
///   >>> min_values = kand.min(prices, 3)
///   ```
#[pyfunction]
#[pyo3(name = "min", signature = (prices, period))]
pub fn min_py(
    py: Python,
    prices: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    // Convert input NumPy array to Rust slice
    let input_prices = prices.as_slice()?;
    let len = input_prices.len();

    // Create output array using vec
    let mut output_min = vec![0.0; len];

    // Perform MIN calculation while releasing the GIL
    py.allow_threads(|| min::min(input_prices, period, &mut output_min))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert output array to Python object
    Ok(output_min.into_pyarray(py).into())
}

/// Calculate the latest MIN value incrementally
///
/// Args:
///   py: Python interpreter token
///   price: Current period's price
///   prev_min: Previous period's MIN value
///   prev_price: Price value being removed from the period
///   period: Period for MIN calculation (must be >= 2)
///
/// Returns:
///   The new MIN value
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> new_min = kand.min_inc(15.0, 12.0, 14.0, 14)
///   ```
#[pyfunction]
#[pyo3(name = "min_inc")]
pub fn min_inc_py(
    py: Python,
    price: TAFloat,
    prev_min: TAFloat,
    prev_price: TAFloat,
    period: usize,
) -> PyResult<TAFloat> {
    // Perform incremental MIN calculation while releasing the GIL
    py.allow_threads(|| min::min_inc(price, prev_min, prev_price, period))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
