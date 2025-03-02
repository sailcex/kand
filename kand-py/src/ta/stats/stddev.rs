use kand::{TAFloat, stats::stddev};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculate Standard Deviation for a NumPy array
///
/// Standard Deviation measures the dispersion of values from their mean over a specified period.
/// It is calculated by taking the square root of the variance.
///
/// Args:
///   input: Input values as a 1-D NumPy array of type `TAFloat`.
///   period: Period for calculation (must be >= 2).
///
/// Returns:
///   A tuple of three 1-D NumPy arrays containing:
///   - Standard Deviation values
///   - Running sum values
///   - Running sum of squares values
///   Each array has the same length as the input, with the first (period-1) elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> prices = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
///   >>> stddev, sum, sum_sq = kand.stddev(prices, 3)
///   ```
#[pyfunction]
#[pyo3(name = "stddev", signature = (input, period))]
pub fn stddev_py(
    py: Python,
    input: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    let input_array = input.as_slice()?;
    let len = input_array.len();

    let mut output_stddev = vec![0.0; len];
    let mut output_sum = vec![0.0; len];
    let mut output_sum_sq = vec![0.0; len];

    py.allow_threads(|| {
        stddev::stddev(
            input_array,
            period,
            &mut output_stddev,
            &mut output_sum,
            &mut output_sum_sq,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output_stddev.into_pyarray(py).into(),
        output_sum.into_pyarray(py).into(),
        output_sum_sq.into_pyarray(py).into(),
    ))
}

/// Calculate the latest Standard Deviation value incrementally
///
/// Args:
///   py: Python interpreter token
///   price: Current period's price
///   prev_sum: Previous period's sum
///   prev_sum_sq: Previous period's sum of squares
///   old_price: Price being removed from the period
///   period: Period for calculation (must be >= 2)
///
/// Returns:
///   A tuple containing:
///   - Latest Standard Deviation value
///   - New sum
///   - New sum of squares
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> stddev, sum, sum_sq = kand.stddev_incremental(
///   ...     10.0,   # current price
///   ...     100.0,  # previous sum
///   ...     1050.0, # previous sum of squares
///   ...     8.0,    # old price
///   ...     14      # period
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "stddev_incremental")]
pub fn stddev_incremental_py(
    py: Python,
    price: TAFloat,
    prev_sum: TAFloat,
    prev_sum_sq: TAFloat,
    old_price: TAFloat,
    period: usize,
) -> PyResult<(TAFloat, TAFloat, TAFloat)> {
    py.allow_threads(|| stddev::stddev_incremental(price, prev_sum, prev_sum_sq, old_price, period))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
