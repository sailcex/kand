use kand::{TAFloat, ohlcv::adr};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Average Daily Range (ADR) over NumPy arrays.
///
/// The ADR measures the average price range over a specified period, helping to identify
/// volatility levels in the market.
///
/// Args:
///     high: High prices as a 1-D NumPy array of type `TAFloat`.
///     low: Low prices as a 1-D NumPy array of type `TAFloat`.
///     period: The time period for ADR calculation (must be >= 2).
///
/// Returns:
///     A new 1-D NumPy array containing the ADR values. The array has the same length as the inputs.
///
/// Examples:
///     ```python
///     >>> import numpy as np
///     >>> import kand
///     >>> high = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
///     >>> low = np.array([8.0, 9.0, 11.0, 10.0, 9.0])
///     >>> period = 3
///     >>> result = kand.adr(high, low, period)
///     ```
#[pyfunction]
#[pyo3(name = "adr", signature = (high, low, period))]
pub fn adr_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    // Convert the input NumPy arrays to Rust slices
    let high_input = high.as_slice()?;
    let low_input = low.as_slice()?;
    let len = high_input.len();

    // Create a new output array using vec
    let mut output = vec![0.0; len];

    // Perform the ADR calculation while releasing the GIL
    py.allow_threads(|| adr::adr(high_input, low_input, period, output.as_mut_slice()))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output array to a Python object
    Ok(output.into_pyarray(py).into())
}

/// Computes the latest Average Daily Range (ADR) value incrementally.
///
/// This function calculates only the latest ADR value using the previous ADR value,
/// avoiding recalculation of the entire series.
///
/// Args:
///     prev_adr: Previous ADR value.
///     new_high: Latest high price.
///     new_low: Latest low price.
///     old_high: Oldest high price to be removed from period.
///     old_low: Oldest low price to be removed from period.
///     period: The time period for ADR calculation (must be >= 2).
///
/// Returns:
///     The latest ADR value.
///
/// Examples:
///     ```python
///     >>> import kand
///     >>> prev_adr = 3.0
///     >>> new_high = 15.0
///     >>> new_low = 12.0
///     >>> old_high = 10.0
///     >>> old_low = 8.0
///     >>> period = 14
///     >>> result = kand.adr_inc(prev_adr, new_high, new_low, old_high, old_low, period)
///     ```
#[pyfunction]
#[pyo3(name = "adr_inc", signature = (prev_adr, new_high, new_low, old_high, old_low, period))]
pub fn adr_inc_py(
    py: Python,
    prev_adr: TAFloat,
    new_high: TAFloat,
    new_low: TAFloat,
    old_high: TAFloat,
    old_low: TAFloat,
    period: usize,
) -> PyResult<TAFloat> {
    // Perform the incremental ADR calculation while releasing the GIL
    py.allow_threads(|| adr::adr_inc(prev_adr, new_high, new_low, old_high, old_low, period))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
