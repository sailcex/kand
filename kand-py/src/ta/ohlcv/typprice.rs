use kand::{TAFloat, ohlcv::typprice};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Typical Price over NumPy arrays.
///
/// The Typical Price is calculated by taking the arithmetic mean of the high, low and close prices
/// for each period.
///
/// Args:
///   high: Input high prices as a 1-D NumPy array of type `TAFloat`.
///   low: Input low prices as a 1-D NumPy array of type `TAFloat`.
///   close: Input close prices as a 1-D NumPy array of type `TAFloat`.
///
/// Returns:
///   A new 1-D NumPy array containing the Typical Price values. The array has the same length as the inputs.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([24.20, 24.07, 24.04])
///   >>> low = np.array([23.85, 23.72, 23.64])
///   >>> close = np.array([23.89, 23.95, 23.67])
///   >>> result = kand.typprice(high, low, close)
///   >>> print(result)
///   [23.98, 23.91, 23.78]
///   ```
#[pyfunction]
#[pyo3(name = "typprice", signature = (high, low, close))]
pub fn typprice_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    // Convert the input NumPy arrays to Rust slices
    let input_high = high.as_slice()?;
    let input_low = low.as_slice()?;
    let input_close = close.as_slice()?;
    let len = input_high.len();

    // Create a new output array using vec
    let mut output = vec![0.0; len];

    // Perform the Typical Price calculation while releasing the GIL to allow other Python threads to run
    py.allow_threads(|| {
        typprice::typprice(input_high, input_low, input_close, output.as_mut_slice())
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output array to a Python object
    Ok(output.into_pyarray(py).into())
}

/// Calculates a single Typical Price value incrementally.
///
/// Args:
///   high: Current period's high price.
///   low: Current period's low price.
///   close: Current period's close price.
///
/// Returns:
///   The calculated Typical Price value.
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> typ_price = kand.typprice_incremental(24.20, 23.85, 23.89)
///   >>> print(typ_price)
///   23.98  # (24.20 + 23.85 + 23.89) / 3
///   ```
#[pyfunction]
#[pyo3(name = "typprice_incremental", signature = (high, low, close))]
pub fn typprice_incremental_py(high: TAFloat, low: TAFloat, close: TAFloat) -> PyResult<TAFloat> {
    typprice::typprice_incremental(high, low, close)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
