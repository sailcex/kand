use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use kand::{ohlcv::apo, utils::types::MAType, TAFloat};

/// Calculate Absolute Price Oscillator (APO) for a NumPy array.
///
/// APO measures the difference between two moving averages of different periods.
/// A positive value indicates the fast MA is above the slow MA, suggesting upward momentum.
/// A negative value indicates the fast MA is below the slow MA, suggesting downward momentum.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   prices: Input prices as a 1-D NumPy array of type `f32`.
///   fast_period: Fast period for MA calculation (must be >= 2).
///   slow_period: Slow period for MA calculation (must be > fast_period).
///   ma_type: Type of Moving Average to use (SMA or EMA).
///
/// Returns:
///   A tuple of 3 1-D NumPy arrays containing:
///   - APO values
///   - Fast MA values
///   - Slow MA values
///   The first (slow_period - 1) elements of each array contain NaN values.
///
/// Note:
///   This function releases the Python GIL during computation using `py.allow_threads()` to enable
///   concurrent Python execution.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> prices = np.array([2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0])
///   >>> apo, fast_ma, slow_ma = kand.apo(prices, 2, 4, kand.MAType.EMA)
///   ```
#[pyfunction]
#[pyo3(name = "apo", signature = (prices, fast_period, slow_period, ma_type))]
pub fn apo_py(
    py: Python,
    prices: PyReadonlyArray1<TAFloat>,
    fast_period: usize,
    slow_period: usize,
    ma_type: MAType,
) -> PyResult<(Py<PyArray1<TAFloat>>, Py<PyArray1<TAFloat>>, Py<PyArray1<TAFloat>>)> {
    let prices_slice = prices.as_slice()?;
    let len = prices_slice.len();

    let mut output_apo = vec![0.0; len];
    let mut output_fast_ma = vec![0.0; len];
    let mut output_slow_ma = vec![0.0; len];

    py.allow_threads(|| {
        apo::apo(
            prices_slice,
            fast_period,
            slow_period,
            ma_type,
            output_apo.as_mut_slice(),
            output_fast_ma.as_mut_slice(),
            output_slow_ma.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output_apo.into_pyarray(py).into(),
        output_fast_ma.into_pyarray(py).into(),
        output_slow_ma.into_pyarray(py).into(),
    ))
}

/// Calculate the next APO value incrementally.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   price: Latest price value.
///   prev_fast_ma: Previous fast MA value.
///   prev_slow_ma: Previous slow MA value.
///   old_fast_price: Price to be removed from fast MA window (for SMA).
///   old_slow_price: Price to be removed from slow MA window (for SMA).
///   fast_period: Fast period for MA calculation (must be >= 2).
///   slow_period: Slow period for MA calculation (must be > fast_period).
///   ma_type: Type of Moving Average to use (SMA or EMA).
///
/// Returns:
///   A tuple containing:
///   - Latest APO value
///   - New fast MA value
///   - New slow MA value
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> apo, fast_ma, slow_ma = kand.apo_incremental(
///   ...     10.0,  # price
///   ...     8.0,   # prev_fast_ma
///   ...     7.0,   # prev_slow_ma
///   ...     6.0,   # old_fast_price
///   ...     5.0,   # old_slow_price
///   ...     2,     # fast_period
///   ...     4,     # slow_period
///   ...     kand.MAType.EMA
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "apo_incremental", signature = (
    price,
    prev_fast_ma,
    prev_slow_ma,
    old_fast_price,
    old_slow_price,
    fast_period,
    slow_period,
    ma_type
))]
pub fn apo_incremental_py(
    py: Python,
    price: TAFloat,
    prev_fast_ma: TAFloat,
    prev_slow_ma: TAFloat,
    old_fast_price: TAFloat,
    old_slow_price: TAFloat,
    fast_period: usize,
    slow_period: usize,
    ma_type: MAType,
) -> PyResult<(TAFloat, TAFloat, TAFloat)> {
    py.allow_threads(|| {
        apo::apo_incremental(
            price,
            prev_fast_ma,
            prev_slow_ma,
            old_fast_price,
            old_slow_price,
            fast_period,
            slow_period,
            ma_type,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
