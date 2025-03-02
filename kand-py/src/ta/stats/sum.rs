use kand::{TAFloat, stats::sum};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculate Sum for a NumPy array
///
/// Calculates the rolling sum of values over a specified period.
///
/// Args:
///   input: Input values as a 1-D NumPy array of type `TAFloat`.
///   period: Period for sum calculation (must be >= 2).
///
/// Returns:
///   A 1-D NumPy array containing the sum values.
///   The first (period-1) elements contain NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> data = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
///   >>> sums = kand.sum(data, 3)
///   ```
#[pyfunction]
#[pyo3(name = "sum", signature = (input, period))]
pub fn sum_py(
    py: Python,
    input: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    // Convert input NumPy array to Rust slice
    let input_data = input.as_slice()?;
    let len = input_data.len();

    // Create output array using vec
    let mut output_sum = vec![0.0; len];

    // Perform sum calculation while releasing the GIL
    py.allow_threads(|| sum::sum(input_data, period, &mut output_sum))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert output array to Python object
    Ok(output_sum.into_pyarray(py).into())
}

/// Calculate the latest sum value incrementally
///
/// Args:
///   py: Python interpreter token
///   new_price: The newest price value to add
///   old_price: The oldest price value to remove
///   prev_sum: The previous sum value
///
/// Returns:
///   The new sum value
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> new_sum = kand.sum_incremental(
///   ...     5.0,    # new price
///   ...     3.0,    # old price
///   ...     10.0,   # previous sum
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "sum_incremental")]
pub fn sum_incremental_py(
    py: Python,
    new_price: TAFloat,
    old_price: TAFloat,
    prev_sum: TAFloat,
) -> PyResult<TAFloat> {
    // Perform incremental sum calculation while releasing the GIL
    py.allow_threads(|| sum::sum_incremental(new_price, old_price, prev_sum))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
