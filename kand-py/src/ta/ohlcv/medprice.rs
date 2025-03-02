use kand::{TAFloat, ohlcv::medprice};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculates the Median Price (MEDPRICE) for a NumPy array.
///
/// The Median Price is a technical analysis indicator that represents the middle point between
/// high and low prices for each period.
///
/// Args:
///   high: Array of high prices as a 1-D NumPy array of type `TAFloat`.
///   low: Array of low prices as a 1-D NumPy array of type `TAFloat`.
///
/// Returns:
///   A 1-D NumPy array containing the median price values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([10.0, 11.0, 12.0])
///   >>> low = np.array([8.0, 9.0, 10.0])
///   >>> result = kand.medprice(high, low)
///   >>> print(result)
///   [9.0, 10.0, 11.0]
///   ```
#[pyfunction]
#[pyo3(name = "medprice")]
pub fn medprice_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    let high_slice = high.as_slice()?;
    let low_slice = low.as_slice()?;
    let len = high_slice.len();
    let mut output = vec![0.0; len];

    py.allow_threads(|| medprice::medprice(high_slice, low_slice, output.as_mut_slice()))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok(output.into_pyarray(py).into())
}

/// Calculates a single Median Price value incrementally.
///
/// Args:
///
///   high: Current period's high price as `f64`.
///   low: Current period's low price as `f64`.
///
/// Returns:
///   The calculated median price value.
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> result = kand.medprice_incremental(10.0, 8.0)
///   >>> print(result)
///   9.0
///   ```
#[pyfunction]
#[pyo3(name = "medprice_incremental")]
pub fn medprice_incremental_py(py: Python, high: TAFloat, low: TAFloat) -> PyResult<TAFloat> {
    py.allow_threads(|| medprice::medprice_incremental(high, low))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
