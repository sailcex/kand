use kand::{TAFloat, ohlcv::aroon};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculate Aroon indicator for a NumPy array.
///
/// The Aroon indicator consists of two lines that measure the time since the last high/low
/// relative to a lookback period. It helps identify the start of new trends and trend reversals.
///
/// Args:
///   high: Input high prices as a 1-D NumPy array of type `TAFloat`.
///   low: Input low prices as a 1-D NumPy array of type `TAFloat`.
///   period: The lookback period for calculations (must be >= 2).
///
/// Returns:
///   A tuple of 6 1-D NumPy arrays containing:
///   - Aroon Up values
///   - Aroon Down values
///   - Previous high values
///   - Previous low values
///   - Days since high values
///   - Days since low values
///   The first (period) elements of each array contain NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
///   >>> low = np.array([8.0, 9.0, 11.0, 10.0, 9.0])
///   >>> aroon_up, aroon_down, prev_high, prev_low, days_high, days_low = kand.aroon(high, low, 3)
///   ```
#[pyfunction]
#[pyo3(name = "aroon", signature = (high, low, period))]
pub fn aroon_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<usize>>,
    Py<PyArray1<usize>>,
)> {
    let high_slice = high.as_slice()?;
    let low_slice = low.as_slice()?;
    let len = high_slice.len();

    let mut output_aroon_up = vec![0.0; len];
    let mut output_aroon_down = vec![0.0; len];
    let mut output_prev_high = vec![0.0; len];
    let mut output_prev_low = vec![0.0; len];
    let mut output_days_since_high = vec![0_usize; len];
    let mut output_days_since_low = vec![0_usize; len];

    py.allow_threads(|| {
        aroon::aroon(
            high_slice,
            low_slice,
            period,
            output_aroon_up.as_mut_slice(),
            output_aroon_down.as_mut_slice(),
            output_prev_high.as_mut_slice(),
            output_prev_low.as_mut_slice(),
            output_days_since_high.as_mut_slice(),
            output_days_since_low.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output_aroon_up.into_pyarray(py).into(),
        output_aroon_down.into_pyarray(py).into(),
        output_prev_high.into_pyarray(py).into(),
        output_prev_low.into_pyarray(py).into(),
        output_days_since_high.into_pyarray(py).into(),
        output_days_since_low.into_pyarray(py).into(),
    ))
}

/// Calculate the next Aroon values incrementally.
///
/// Args:
///
///   high: Current period's high price.
///   low: Current period's low price.
///   prev_high: Previous highest price in period.
///   prev_low: Previous lowest price in period.
///   days_since_high: Days since previous highest price.
///   days_since_low: Days since previous lowest price.
///   period: The lookback period (must be >= 2).
///
/// Returns:
///   A tuple containing:
///   - Aroon Up value
///   - Aroon Down value
///   - New highest price
///   - New lowest price
///   - Updated days since high
///   - Updated days since low
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> aroon_up, aroon_down, new_high, new_low, days_high, days_low = kand.aroon_inc(
///   ...     15.0,  # high
///   ...     12.0,  # low
///   ...     14.0,  # prev_high
///   ...     11.0,  # prev_low
///   ...     2,     # days_since_high
///   ...     1,     # days_since_low
///   ...     14     # period
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "aroon_inc", signature = (
    high,
    low,
    prev_high,
    prev_low,
    days_since_high,
    days_since_low,
    period
))]
pub fn aroon_inc_py(
    py: Python,
    high: TAFloat,
    low: TAFloat,
    prev_high: TAFloat,
    prev_low: TAFloat,
    days_since_high: usize,
    days_since_low: usize,
    period: usize,
) -> PyResult<(TAFloat, TAFloat, TAFloat, TAFloat, usize, usize)> {
    py.allow_threads(|| {
        aroon::aroon_inc(
            high,
            low,
            prev_high,
            prev_low,
            days_since_high,
            days_since_low,
            period,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
