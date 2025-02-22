use kand::{ohlcv::dema, TAFloat};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculates Double Exponential Moving Average (DEMA) over NumPy arrays.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   input_price: Price values as a 1-D NumPy array of type `f32`.
///   period: Smoothing period for EMA calculations. Must be >= 2.
///
/// Returns:
///   A tuple of 1-D NumPy arrays containing:
///   - DEMA values
///   - First EMA values
///   - Second EMA values
///   Each array has the same length as the input, with the first `2*(period-1)` elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> prices = np.array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0])
///   >>> dema, ema1, ema2 = kand.dema(prices, 3)
///   ```
#[pyfunction]
#[pyo3(name = "dema", signature = (input_price, period))]
pub fn dema_py(
    py: Python,
    input_price: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    let input = input_price.as_slice()?;
    let len = input.len();

    let mut output_dema = vec![0.0; len];
    let mut output_ema1 = vec![0.0; len];
    let mut output_ema2 = vec![0.0; len];

    py.allow_threads(|| {
        dema::dema(
            input,
            period,
            output_dema.as_mut_slice(),
            output_ema1.as_mut_slice(),
            output_ema2.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output_dema.into_pyarray(py).into(),
        output_ema1.into_pyarray(py).into(),
        output_ema2.into_pyarray(py).into(),
    ))
}

/// Calculates the next DEMA value incrementally.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   price: Current price value.
///   prev_ema1: Previous value of first EMA.
///   prev_ema2: Previous value of second EMA.
///   period: Smoothing period. Must be >= 2.
///
/// Returns:
///   A tuple containing (DEMA, new_ema1, new_ema2).
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> dema, ema1, ema2 = kand.dema_incremental(10.0, 9.5, 9.0, 3)
///   ```
#[pyfunction]
#[pyo3(name = "dema_incremental", signature = (price, prev_ema1, prev_ema2, period))]
pub fn dema_incremental_py(
    py: Python,
    price: TAFloat,
    prev_ema1: TAFloat,
    prev_ema2: TAFloat,
    period: usize,
) -> PyResult<(TAFloat, TAFloat, TAFloat)> {
    py.allow_threads(|| dema::dema_incremental(price, prev_ema1, prev_ema2, period))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
