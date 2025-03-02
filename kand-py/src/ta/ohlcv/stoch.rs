use kand::{TAFloat, ohlcv::stoch};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Stochastic Oscillator indicator over NumPy arrays.
///
/// The Stochastic Oscillator is a momentum indicator that shows the location of the close
/// relative to the high-low range over a set number of periods. The indicator consists of
/// two lines: %K (the fast line) and %D (the slow line).
///
/// Args:
///     high: High prices as a 1-D NumPy array of type `TAFloat`.
///     low: Low prices as a 1-D NumPy array of type `TAFloat`.
///     close: Close prices as a 1-D NumPy array of type `TAFloat`.
///     k_period: Period for %K calculation. Must be >= 2.
///     k_slow_period: Smoothing period for slow %K. Must be >= 2.
///     d_period: Period for %D calculation. Must be >= 2.
///
/// Returns:
///     A tuple of three 1-D NumPy arrays containing:
///     - Fast %K values
///     - Slow %K values
///     - %D values
///     Each array has the same length as the input, with initial values being NaN.
///
/// Examples:
///     ```python
///     >>> import numpy as np
///     >>> import kand
///     >>> high = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
///     >>> low = np.array([8.0, 9.0, 11.0, 10.0, 9.0])
///     >>> close = np.array([9.0, 11.0, 14.0, 12.0, 11.0])
///     >>> fast_k, k, d = kand.stoch(high, low, close, 3, 2, 2)
///     ```
#[pyfunction]
#[pyo3(name = "stoch", signature = (high, low, close, k_period, k_slow_period, d_period))]
pub fn stoch_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
    k_period: usize,
    k_slow_period: usize,
    d_period: usize,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    let high_slice = high.as_slice()?;
    let low_slice = low.as_slice()?;
    let close_slice = close.as_slice()?;
    let len = high_slice.len();

    let mut output_fast_k = vec![0.0; len];
    let mut output_k = vec![0.0; len];
    let mut output_d = vec![0.0; len];

    py.allow_threads(|| {
        stoch::stoch(
            high_slice,
            low_slice,
            close_slice,
            k_period,
            k_slow_period,
            d_period,
            output_fast_k.as_mut_slice(),
            output_k.as_mut_slice(),
            output_d.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output_fast_k.into_pyarray(py).into(),
        output_k.into_pyarray(py).into(),
        output_d.into_pyarray(py).into(),
    ))
}
