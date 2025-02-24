use kand::{TAFloat, ohlcv::adx};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculate Average Directional Index (ADX) for a NumPy array
///
/// The ADX (Average Directional Index) measures the strength of a trend, regardless of whether it's up or down.
/// Values range from 0 to 100, with higher values indicating stronger trends.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   high: High prices as a 1-D NumPy array of type `f64`.
///   low: Low prices as a 1-D NumPy array of type `f64`.
///   close: Close prices as a 1-D NumPy array of type `f64`.
///   period: Period for ADX calculation (typically 14). Must be positive.
///
/// Returns:
///   A tuple of four 1-D NumPy arrays containing:
///   - ADX values
///   - Smoothed +DM values
///   - Smoothed -DM values
///   - Smoothed TR values
///   Each array has the same length as the input, with the first (2*period-1) elements containing NaN values.
///
/// Note:
///   This function releases the Python GIL during computation using `py.allow_threads()` to enable
///   concurrent Python execution.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([24.20, 24.07, 24.04, 23.87, 23.67])
///   >>> low = np.array([23.85, 23.72, 23.64, 23.37, 23.46])
///   >>> close = np.array([23.89, 23.95, 23.67, 23.78, 23.50])
///   >>> adx, plus_dm, minus_dm, tr = kand.adx(high, low, close, 2)
///   ```
#[pyfunction]
#[pyo3(name = "adx", signature = (high, low, close, period))]
pub fn adx_py(
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
)> {
    // Convert input NumPy arrays to Rust slices
    let input_high = high.as_slice()?;
    let input_low = low.as_slice()?;
    let input_close = close.as_slice()?;
    let len = input_high.len();

    // Create output arrays using vec
    let mut output_adx = vec![0.0; len];
    let mut output_smoothed_plus_dm = vec![0.0; len];
    let mut output_smoothed_minus_dm = vec![0.0; len];
    let mut output_smoothed_tr = vec![0.0; len];

    // Perform ADX calculation while releasing the GIL
    py.allow_threads(|| {
        adx::adx(
            input_high,
            input_low,
            input_close,
            period,
            &mut output_adx,
            &mut output_smoothed_plus_dm,
            &mut output_smoothed_minus_dm,
            &mut output_smoothed_tr,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert output arrays to Python objects
    Ok((
        output_adx.into_pyarray(py).into(),
        output_smoothed_plus_dm.into_pyarray(py).into(),
        output_smoothed_minus_dm.into_pyarray(py).into(),
        output_smoothed_tr.into_pyarray(py).into(),
    ))
}

/// Calculate the latest ADX value incrementally
///
/// Args:
///   py: Python interpreter token
///   high: Current period's high price
///   low: Current period's low price
///   prev_high: Previous period's high price
///   prev_low: Previous period's low price
///   prev_close: Previous period's close price
///   prev_adx: Previous period's ADX value
///   prev_smoothed_plus_dm: Previous period's smoothed +DM
///   prev_smoothed_minus_dm: Previous period's smoothed -DM
///   prev_smoothed_tr: Previous period's smoothed TR
///   period: Period for ADX calculation (typically 14)
///
/// Returns:
///   A tuple containing:
///   - Latest ADX value
///   - New smoothed +DM
///   - New smoothed -DM
///   - New smoothed TR
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> adx, plus_dm, minus_dm, tr = kand.adx_incremental(
///   ...     24.20,  # current high
///   ...     23.85,  # current low
///   ...     24.07,  # previous high
///   ...     23.72,  # previous low
///   ...     23.95,  # previous close
///   ...     25.0,   # previous ADX
///   ...     0.5,    # previous smoothed +DM
///   ...     0.3,    # previous smoothed -DM
///   ...     1.2,    # previous smoothed TR
///   ...     14      # period
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "adx_incremental")]
pub fn adx_incremental_py(
    py: Python,
    high: TAFloat,
    low: TAFloat,
    prev_high: TAFloat,
    prev_low: TAFloat,
    prev_close: TAFloat,
    prev_adx: TAFloat,
    prev_smoothed_plus_dm: TAFloat,
    prev_smoothed_minus_dm: TAFloat,
    prev_smoothed_tr: TAFloat,
    period: usize,
) -> PyResult<(TAFloat, TAFloat, TAFloat, TAFloat)> {
    // Perform incremental ADX calculation while releasing the GIL
    py.allow_threads(|| {
        adx::adx_incremental(
            high,
            low,
            prev_high,
            prev_low,
            prev_close,
            prev_adx,
            prev_smoothed_plus_dm,
            prev_smoothed_minus_dm,
            prev_smoothed_tr,
            period,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
