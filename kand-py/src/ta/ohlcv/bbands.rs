use kand::{TAFloat, ohlcv::bbands};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculate Bollinger Bands for a NumPy array.
///
/// Bollinger Bands consist of:
/// - A middle band (N-period simple moving average)
/// - An upper band (K standard deviations above middle band)
/// - A lower band (K standard deviations below middle band)
///
/// Args:
///   price: Input price values as a 1-D NumPy array of type `TAFloat`.
///   period: The time period for calculations (must be >= 2).
///   dev_up: Number of standard deviations for upper band.
///   dev_down: Number of standard deviations for lower band.
///
/// Returns:
///   A tuple of 7 1-D NumPy arrays containing:
///   - Upper band values
///   - Middle band values
///   - Lower band values
///   - SMA values
///   - Variance values
///   - Sum values
///   - Sum of squares values
///   The first (period-1) elements of each array contain NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> price = np.array([10.0, 11.0, 12.0, 13.0, 14.0])
///   >>> upper, middle, lower, sma, var, sum, sum_sq = kand.bbands(price, 3, 2.0, 2.0)
///   ```
#[pyfunction]
#[pyo3(name = "bbands", signature = (price, period, dev_up, dev_down))]
pub fn bbands_py(
    py: Python,
    price: PyReadonlyArray1<TAFloat>,
    period: usize,
    dev_up: TAFloat,
    dev_down: TAFloat,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    let price_slice = price.as_slice()?;
    let len = price_slice.len();

    let mut output_upper = vec![0.0; len];
    let mut output_middle = vec![0.0; len];
    let mut output_lower = vec![0.0; len];
    let mut output_sma = vec![0.0; len];
    let mut output_var = vec![0.0; len];
    let mut output_sum = vec![0.0; len];
    let mut output_sum_sq = vec![0.0; len];

    py.allow_threads(|| {
        bbands::bbands(
            price_slice,
            period,
            dev_up,
            dev_down,
            output_upper.as_mut_slice(),
            output_middle.as_mut_slice(),
            output_lower.as_mut_slice(),
            output_sma.as_mut_slice(),
            output_var.as_mut_slice(),
            output_sum.as_mut_slice(),
            output_sum_sq.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output_upper.into_pyarray(py).into(),
        output_middle.into_pyarray(py).into(),
        output_lower.into_pyarray(py).into(),
        output_sma.into_pyarray(py).into(),
        output_var.into_pyarray(py).into(),
        output_sum.into_pyarray(py).into(),
        output_sum_sq.into_pyarray(py).into(),
    ))
}

/// Calculate the next Bollinger Bands values incrementally.
///
/// Args:
///
///   price: The current price value.
///   prev_sma: The previous SMA value.
///   prev_sum: The previous sum for variance calculation.
///   prev_sum_sq: The previous sum of squares for variance calculation.
///   old_price: The oldest price value to be removed from the period.
///   period: The time period for calculations (must be >= 2).
///   dev_up: Number of standard deviations for upper band.
///   dev_down: Number of standard deviations for lower band.
///
/// Returns:
///   A tuple containing:
///   - Upper Band value
///   - Middle Band value
///   - Lower Band value
///   - New SMA value
///   - New Sum value
///   - New Sum of Squares value
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> upper, middle, lower, sma, sum, sum_sq = kand.bbands_incremental(
///   ...     10.0,   # price
///   ...     9.5,    # prev_sma
///   ...     28.5,   # prev_sum
///   ...     272.25, # prev_sum_sq
///   ...     9.0,    # old_price
///   ...     3,      # period
///   ...     2.0,    # dev_up
///   ...     2.0     # dev_down
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "bbands_incremental", signature = (
    price,
    prev_sma,
    prev_sum,
    prev_sum_sq,
    old_price,
    period,
    dev_up,
    dev_down
))]
pub fn bbands_incremental_py(
    py: Python,
    price: TAFloat,
    prev_sma: TAFloat,
    prev_sum: TAFloat,
    prev_sum_sq: TAFloat,
    old_price: TAFloat,
    period: usize,
    dev_up: TAFloat,
    dev_down: TAFloat,
) -> PyResult<(TAFloat, TAFloat, TAFloat, TAFloat, TAFloat, TAFloat)> {
    py.allow_threads(|| {
        bbands::bbands_incremental(
            price,
            prev_sma,
            prev_sum,
            prev_sum_sq,
            old_price,
            period,
            dev_up,
            dev_down,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
