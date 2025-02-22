use kand::{ohlcv::obv, TAFloat};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the On Balance Volume (OBV) over NumPy arrays.
///
/// On Balance Volume (OBV) is a momentum indicator that uses volume flow to predict changes in stock price.
/// When volume increases without a significant price change, the price will eventually jump upward.
/// When volume decreases without a significant price change, the price will eventually jump downward.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   close: Close prices as a 1-D NumPy array of type `f64`.
///   volume: Volume data as a 1-D NumPy array of type `f64`.
///
/// Returns:
///   A new 1-D NumPy array containing the OBV values. The array has the same length as the input.
///
/// Note:
///   This function releases the Python GIL during computation using `py.allow_threads()` to enable
///   concurrent Python execution.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> close = np.array([10.0, 12.0, 11.0, 13.0])
///   >>> volume = np.array([100.0, 150.0, 120.0, 200.0])
///   >>> result = kand.obv(close, volume)
///   >>> print(result)
///   [100.0, 250.0, 130.0, 330.0]
///   ```
#[pyfunction]
#[pyo3(name = "obv", signature = (close, volume))]
pub fn obv_py(
    py: Python,
    close: PyReadonlyArray1<TAFloat>,
    volume: PyReadonlyArray1<TAFloat>,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    // Convert the input NumPy arrays to Rust slices
    let input_close = close.as_slice()?;
    let input_volume = volume.as_slice()?;
    let len = input_close.len();

    // Create a new output array using vec
    let mut output = vec![0.0; len];

    // Perform the OBV calculation while releasing the GIL to allow other Python threads to run
    py.allow_threads(|| obv::obv(input_close, input_volume, output.as_mut_slice()))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output array to a Python object
    Ok(output.into_pyarray(py).into())
}

/// Calculates the next OBV value incrementally.
///
/// This function provides an optimized way to calculate a single new OBV value
/// using the previous OBV value and current price/volume data.
///
/// Args:
///   curr_close: Current closing price as `f64`.
///   prev_close: Previous closing price as `f64`.
///   volume: Current volume as `f64`.
///   prev_obv: Previous OBV value as `f64`.
///
/// Returns:
///   The calculated OBV value.
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> curr_close = 12.0
///   >>> prev_close = 10.0
///   >>> volume = 150.0
///   >>> prev_obv = 100.0
///   >>> result = kand.obv_incremental(curr_close, prev_close, volume, prev_obv)
///   >>> print(result)
///   250.0
///   ```
#[pyfunction]
#[pyo3(name = "obv_incremental", signature = (curr_close, prev_close, volume, prev_obv))]
pub fn obv_incremental_py(
    curr_close: TAFloat,
    prev_close: TAFloat,
    volume: TAFloat,
    prev_obv: TAFloat,
) -> PyResult<TAFloat> {
    obv::obv_incremental(curr_close, prev_close, volume, prev_obv)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
