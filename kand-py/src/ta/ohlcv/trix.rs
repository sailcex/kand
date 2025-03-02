use kand::{TAFloat, ohlcv::trix};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculates the Triple Exponential Moving Average Oscillator (TRIX) over a NumPy array.
///
/// TRIX is a momentum oscillator that measures the rate of change of a triple exponentially smoothed moving average.
/// It helps identify oversold and overbought conditions and potential trend reversals through divergences.
///
/// Args:
///   prices: Input prices as a 1-D NumPy array of type `TAFloat`.
///   period: Period for EMA calculations (must be >= 2).
///
/// Returns:
///   A tuple of 4 1-D NumPy arrays containing:
///   - TRIX values
///   - First EMA values
///   - Second EMA values
///   - Third EMA values
///   The first lookback elements of each array contain NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> prices = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
///   >>> trix, ema1, ema2, ema3 = kand.trix(prices, 2)
///   ```
#[pyfunction]
#[pyo3(name = "trix", signature = (prices, period))]
pub fn trix_py(
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

    let mut output = vec![0.0; len];
    let mut ema1 = vec![0.0; len];
    let mut ema2 = vec![0.0; len];
    let mut ema3 = vec![0.0; len];

    py.allow_threads(|| {
        trix::trix(
            prices_slice,
            period,
            output.as_mut_slice(),
            ema1.as_mut_slice(),
            ema2.as_mut_slice(),
            ema3.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output.into_pyarray(py).into(),
        ema1.into_pyarray(py).into(),
        ema2.into_pyarray(py).into(),
        ema3.into_pyarray(py).into(),
    ))
}

/// Calculates a single new TRIX value incrementally.
///
/// Args:
///   price: Current price value.
///   prev_ema1: Previous first EMA value.
///   prev_ema2: Previous second EMA value.
///   prev_ema3: Previous third EMA value.
///   period: Period for EMA calculations (must be >= 2).
///
/// Returns:
///   A tuple containing:
///   - TRIX value
///   - Updated first EMA value
///   - Updated second EMA value
///   - Updated third EMA value
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> trix, ema1, ema2, ema3 = kand.trix_incremental(
///   ...     100.0,  # price
///   ...     98.0,   # prev_ema1
///   ...     97.0,   # prev_ema2
///   ...     96.0,   # prev_ema3
///   ...     14      # period
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "trix_incremental", signature = (price, prev_ema1, prev_ema2, prev_ema3, period))]
pub fn trix_incremental_py(
    price: TAFloat,
    prev_ema1: TAFloat,
    prev_ema2: TAFloat,
    prev_ema3: TAFloat,
    period: usize,
) -> PyResult<(TAFloat, TAFloat, TAFloat, TAFloat)> {
    trix::trix_incremental(price, prev_ema1, prev_ema2, prev_ema3, period)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
