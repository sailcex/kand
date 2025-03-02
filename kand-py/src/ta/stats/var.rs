use kand::{TAFloat, stats::var};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculate Variance (VAR) for a NumPy array
///
/// Variance measures the average squared deviation of data points from their mean over a specified period.
///
/// Args:
///   prices: Input prices as a 1-D NumPy array of type `TAFloat`.
///   period: Period for Variance calculation (must be >= 2).
///
/// Returns:
///   A tuple of three 1-D NumPy arrays containing:
///   - Variance values
///   - Running sum values
///   - Running sum of squares values
///   Each array has the same length as the input, with the first (period-1) elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> prices = np.array([2.0, 4.0, 6.0, 8.0, 10.0])
///   >>> var, sum, sum_sq = kand.var(prices, 3)
///   ```
#[pyfunction]
#[pyo3(name = "var", signature = (prices, period))]
pub fn var_py(
    py: Python,
    prices: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    let input_prices = prices.as_slice()?;
    let len = input_prices.len();

    let mut output_var = vec![0.0; len];
    let mut output_sum = vec![0.0; len];
    let mut output_sum_sq = vec![0.0; len];

    py.allow_threads(|| {
        var::var(
            input_prices,
            period,
            &mut output_var,
            &mut output_sum,
            &mut output_sum_sq,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output_var.into_pyarray(py).into(),
        output_sum.into_pyarray(py).into(),
        output_sum_sq.into_pyarray(py).into(),
    ))
}

/// Calculate the latest Variance value incrementally
///
/// Args:
///   py: Python interpreter token
///   price: Current period's price
///   prev_sum: Previous period's sum
///   prev_sum_sq: Previous period's sum of squares
///   old_price: Price being removed from the period
///   period: Period for Variance calculation (must be >= 2)
///
/// Returns:
///   A tuple containing:
///   - Latest Variance value
///   - New sum
///   - New sum of squares
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> var, sum, sum_sq = kand.var_incremental(
///   ...     10.0,  # current price
///   ...     25.0,  # previous sum
///   ...     220.0, # previous sum of squares
///   ...     5.0,   # price to remove
///   ...     3      # period
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "var_incremental")]
pub fn var_incremental_py(
    py: Python,
    price: TAFloat,
    prev_sum: TAFloat,
    prev_sum_sq: TAFloat,
    old_price: TAFloat,
    period: usize,
) -> PyResult<(TAFloat, TAFloat, TAFloat)> {
    py.allow_threads(|| var::var_incremental(price, prev_sum, prev_sum_sq, old_price, period))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
