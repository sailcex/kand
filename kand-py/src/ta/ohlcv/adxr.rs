use kand::{TAFloat, ohlcv::adxr};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculate Average Directional Index Rating (ADXR) for a NumPy array.
///
/// ADXR is a momentum indicator that measures the strength of a trend by comparing
/// the current ADX value with the ADX value from `period` days ago.
///
/// Args:
///   high: High prices as a 1-D NumPy array of type `TAFloat`.
///   low: Low prices as a 1-D NumPy array of type `TAFloat`.
///   close: Close prices as a 1-D NumPy array of type `TAFloat`.
///   period: Period for ADX calculation (typically 14).
///
/// Returns:
///   A tuple of 5 1-D NumPy arrays containing:
///   - ADXR values
///   - ADX values
///   - Smoothed +DM values
///   - Smoothed -DM values
///   - Smoothed TR values
///   The first (3*period-2) elements of each array contain NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([24.20, 24.07, 24.04, 23.87, 23.67])
///   >>> low = np.array([23.85, 23.72, 23.64, 23.37, 23.46])
///   >>> close = np.array([23.89, 23.95, 23.67, 23.78, 23.50])
///   >>> adxr, adx, plus_dm, minus_dm, tr = kand.adxr(high, low, close, 2)
///   ```
#[pyfunction]
#[pyo3(name = "adxr", signature = (high, low, close, period))]
pub fn adxr_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    let high_slice = high.as_slice()?;
    let low_slice = low.as_slice()?;
    let close_slice = close.as_slice()?;
    let len = high_slice.len();

    let mut output_adxr = vec![0.0; len];
    let mut output_adx = vec![0.0; len];
    let mut output_plus_dm = vec![0.0; len];
    let mut output_minus_dm = vec![0.0; len];
    let mut output_tr = vec![0.0; len];

    py.allow_threads(|| {
        adxr::adxr(
            high_slice,
            low_slice,
            close_slice,
            period,
            &mut output_adxr,
            &mut output_adx,
            &mut output_plus_dm,
            &mut output_minus_dm,
            &mut output_tr,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output_adxr.into_pyarray(py).into(),
        output_adx.into_pyarray(py).into(),
        output_plus_dm.into_pyarray(py).into(),
        output_minus_dm.into_pyarray(py).into(),
        output_tr.into_pyarray(py).into(),
    ))
}

/// Calculate the latest ADXR value incrementally
///
/// Args:
///
///   high: Current high price as TAFloat.
///   low: Current low price as TAFloat.
///   prev_high: Previous high price as TAFloat.
///   prev_low: Previous low price as TAFloat.
///   prev_close: Previous close price as TAFloat.
///   prev_adx: Previous ADX value as TAFloat.
///   prev_adx_period_ago: ADX value from period days ago as TAFloat.
///   prev_smoothed_plus_dm: Previous smoothed +DM value as TAFloat.
///   prev_smoothed_minus_dm: Previous smoothed -DM value as TAFloat.
///   prev_smoothed_tr: Previous smoothed TR value as TAFloat.
///   period: Period for ADX calculation (typically 14).
///
/// Returns:
///   A tuple of 5 values:
///   - Latest ADXR value
///   - Latest ADX value
///   - New smoothed +DM value
///   - New smoothed -DM value
///   - New smoothed TR value
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> adxr, adx, plus_dm, minus_dm, tr = kand.adxr_inc(
///   ...     24.20,  # high
///   ...     23.85,  # low
///   ...     24.07,  # prev_high
///   ...     23.72,  # prev_low
///   ...     23.95,  # prev_close
///   ...     25.0,   # prev_adx
///   ...     20.0,   # prev_adx_period_ago
///   ...     0.5,    # prev_smoothed_plus_dm
///   ...     0.3,    # prev_smoothed_minus_dm
///   ...     1.2,    # prev_smoothed_tr
///   ...     14      # period
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "adxr_inc", signature = (
    high,
    low,
    prev_high,
    prev_low,
    prev_close,
    prev_adx,
    prev_adx_period_ago,
    prev_smoothed_plus_dm,
    prev_smoothed_minus_dm,
    prev_smoothed_tr,
    period
))]
pub fn adxr_inc_py(
    py: Python,
    high: TAFloat,
    low: TAFloat,
    prev_high: TAFloat,
    prev_low: TAFloat,
    prev_close: TAFloat,
    prev_adx: TAFloat,
    prev_adx_period_ago: TAFloat,
    prev_smoothed_plus_dm: TAFloat,
    prev_smoothed_minus_dm: TAFloat,
    prev_smoothed_tr: TAFloat,
    period: usize,
) -> PyResult<(TAFloat, TAFloat, TAFloat, TAFloat, TAFloat)> {
    py.allow_threads(|| {
        adxr::adxr_inc(
            high,
            low,
            prev_high,
            prev_low,
            prev_close,
            prev_adx,
            prev_adx_period_ago,
            prev_smoothed_plus_dm,
            prev_smoothed_minus_dm,
            prev_smoothed_tr,
            period,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
