use kand::{TAFloat, ohlcv::tema};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculate Triple Exponential Moving Average (TEMA) for a NumPy array.
///
/// TEMA is an enhanced moving average designed to reduce lag while maintaining smoothing properties.
/// It applies triple exponential smoothing to put more weight on recent data and less on older data.
///
/// Args:
///   prices: Input prices as a 1-D NumPy array of type `TAFloat`.
///   period: Smoothing period for calculations (must be >= 2).
///
/// Returns:
///   A tuple of 4 1-D NumPy arrays containing:
///   - TEMA values
///   - First EMA values
///   - Second EMA values
///   - Third EMA values
///   The first (3 * (period - 1)) elements of each array contain NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> prices = np.array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0])
///   >>> tema, ema1, ema2, ema3 = kand.tema(prices, 3)
///   ```
#[pyfunction]
#[pyo3(name = "tema", signature = (prices, period))]
pub fn tema_py(
    py: Python,
    prices: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    let prices_slice = prices.as_slice()?;
    let len = prices_slice.len();

    let mut output_tema = vec![0.0; len];
    let mut output_ema1 = vec![0.0; len];
    let mut output_ema2 = vec![0.0; len];
    let mut output_ema3 = vec![0.0; len];

    py.allow_threads(|| {
        tema::tema(
            prices_slice,
            period,
            output_tema.as_mut_slice(),
            output_ema1.as_mut_slice(),
            output_ema2.as_mut_slice(),
            output_ema3.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output_tema.into_pyarray(py).into(),
        output_ema1.into_pyarray(py).into(),
        output_ema2.into_pyarray(py).into(),
        output_ema3.into_pyarray(py).into(),
    ))
}

/// Calculate the next TEMA value incrementally.
///
/// Args:
///   new_price: Latest price value to process.
///   prev_ema1: Previous value of first EMA.
///   prev_ema2: Previous value of second EMA.
///   prev_ema3: Previous value of third EMA.
///   period: Smoothing period for calculations (must be >= 2).
///
/// Returns:
///   A tuple containing:
///   - Current TEMA value
///   - Updated first EMA
///   - Updated second EMA
///   - Updated third EMA
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> tema, ema1, ema2, ema3 = kand.tema_inc(
///   ...     10.0,  # new_price
///   ...     9.0,   # prev_ema1
///   ...     8.0,   # prev_ema2
///   ...     7.0,   # prev_ema3
///   ...     3      # period
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "tema_inc", signature = (new_price, prev_ema1, prev_ema2, prev_ema3, period))]
pub fn tema_inc_py(
    new_price: TAFloat,
    prev_ema1: TAFloat,
    prev_ema2: TAFloat,
    prev_ema3: TAFloat,
    period: usize,
) -> PyResult<(TAFloat, TAFloat, TAFloat, TAFloat)> {
    tema::tema_inc(new_price, prev_ema1, prev_ema2, prev_ema3, period)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
