use kand::{ohlcv::trange, TAFloat};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the True Range (TR) over NumPy arrays.
///
/// True Range measures the market's volatility by considering the current high-low range
/// and the previous close price.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   high: High prices as a 1-D NumPy array of type `f32`.
///   low: Low prices as a 1-D NumPy array of type `f32`.
///   close: Close prices as a 1-D NumPy array of type `f32`.
///
/// Returns:
///   A new 1-D NumPy array containing the TR values. The array has the same length as the input,
///   with the first element containing NaN value.
///
/// Note:
///   This function releases the Python GIL during computation using `py.allow_threads()` to enable
///   concurrent Python execution.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([10.0, 12.0, 15.0])
///   >>> low = np.array([8.0, 9.0, 11.0])
///   >>> close = np.array([9.0, 11.0, 14.0])
///   >>> result = kand.trange(high, low, close)
///   >>> print(result)
///   [nan, 3.0, 4.0]
///   ```
#[pyfunction]
#[pyo3(name = "trange", signature = (high, low, close))]
pub fn trange_py(
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

    // Perform the TR calculation while releasing the GIL to allow other Python threads to run
    py.allow_threads(|| trange::trange(input_high, input_low, input_close, output.as_mut_slice()))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output array to a Python object
    Ok(output.into_pyarray(py).into())
}

/// Calculates a single True Range value for the most recent period.
///
/// Args:
///   high: Current period's high price.
///   low: Current period's low price.
///   prev_close: Previous period's closing price.
///
/// Returns:
///   The calculated True Range value.
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> tr = kand.trange_incremental(12.0, 9.0, 11.0)
///   >>> print(tr)
///   3.0  # max(3, 1, 2)
///   ```
#[pyfunction]
#[pyo3(name = "trange_incremental", signature = (high, low, prev_close))]
pub fn trange_incremental_py(
    high: TAFloat,
    low: TAFloat,
    prev_close: TAFloat,
) -> PyResult<TAFloat> {
    trange::trange_incremental(high, low, prev_close)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
