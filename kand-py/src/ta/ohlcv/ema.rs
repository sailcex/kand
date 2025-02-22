use kand::{ohlcv::ema, TAFloat};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Exponential Moving Average (EMA) over a NumPy array.
///
/// The Exponential Moving Average is calculated by applying more weight to recent prices
/// via a smoothing factor k. Each value is calculated as:
/// EMA = Price * k + EMA(previous) * (1 - k)
/// where k is typically 2/(period+1).
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   data: Input data as a 1-D NumPy array of type `f32`.
///   period: Window size for EMA calculation. Must be positive and less than input length.
///   k: Optional custom smoothing factor. If None, uses default k = 2/(period+1).
///
/// Returns:
///   A new 1-D NumPy array containing the EMA values. The array has the same length as the input,
///   with the first `period-1` elements containing NaN values.
///
/// Note:
///   This function releases the Python GIL during computation using `py.allow_threads()` to enable
///   concurrent Python execution.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> data = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
///   >>> result = kand.ema(data, 3)
///   >>> print(result)
///   [nan, nan, 2.0, 3.0, 4.2]
///   ```
#[pyfunction]
#[pyo3(name = "ema", signature = (data, period, k=None))]
pub fn ema_py(
    py: Python,
    data: PyReadonlyArray1<TAFloat>,
    period: usize,
    k: Option<TAFloat>,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    // Convert the input NumPy array to a Rust slice.
    let input = data.as_slice()?;
    let len = input.len();

    // Create a new output array using vec
    let mut output = vec![0.0; len];

    // Perform the EMA calculation while releasing the GIL to allow other Python threads to run.
    py.allow_threads(|| ema::ema(input, period, k, output.as_mut_slice()))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output array to a Python object
    Ok(output.into_pyarray(py).into())
}

/// Computes the latest EMA value incrementally.
///
/// This function provides an efficient way to calculate EMA values for new data without
/// reprocessing the entire dataset.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   price: Current period's price value as `f32`.
///   prev_ema: Previous period's EMA value as `f32`.
///   period: Window size for EMA calculation. Must be >= 2.
///   k: Optional custom smoothing factor. If None, uses default k = 2/(period+1).
///
/// Returns:
///   The new EMA value as `f32`.
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> current_price = 15.0
///   >>> prev_ema = 14.5
///   >>> period = 14
///   >>> new_ema = kand.ema_incremental(current_price, prev_ema, period)
///   ```
#[pyfunction]
#[pyo3(name = "ema_incremental", signature = (price, prev_ema, period, k=None))]
pub fn ema_incremental_py(
    py: Python,
    price: TAFloat,
    prev_ema: TAFloat,
    period: usize,
    k: Option<TAFloat>,
) -> PyResult<TAFloat> {
    py.allow_threads(|| {
        ema::ema_incremental(price, prev_ema, period, k)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    })
}
