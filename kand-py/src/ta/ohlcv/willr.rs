use kand::{TAFloat, ohlcv::willr};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculates Williams %R (Williams Percent Range) for a series of prices.
///
/// Williams %R is a momentum indicator that measures overbought and oversold levels by comparing
/// the closing price to the high-low range over a specified period. The indicator oscillates
/// between 0 and -100.
///
/// Args:
///     high: Input high prices as a 1-D NumPy array of type `TAFloat`.
///     low: Input low prices as a 1-D NumPy array of type `TAFloat`.
///     close: Input closing prices as a 1-D NumPy array of type `TAFloat`.
///     period: Lookback period for calculations. Must be >= 2.
///
/// Returns:
///     A tuple of three 1-D NumPy arrays containing:
///     - Williams %R values
///     - Highest high values for each period
///     - Lowest low values for each period
///     Each array has the same length as the input, with the first `period-1` elements containing NaN values.
///
/// Examples:
///     ```python
///     >>> import numpy as np
///     >>> import kand
///     >>> high = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
///     >>> low = np.array([8.0, 9.0, 11.0, 10.0, 9.0])
///     >>> close = np.array([9.0, 11.0, 14.0, 12.0, 11.0])
///     >>> willr, highest, lowest = kand.willr(high, low, close, 3)
///     ```
#[pyfunction]
#[pyo3(name = "willr", signature = (high, low, close, period))]
pub fn willr_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    let high_slice = high.as_slice()?;
    let low_slice = low.as_slice()?;
    let close_slice = close.as_slice()?;
    let len = high_slice.len();

    let mut output = vec![0.0; len];
    let mut output_highest = vec![0.0; len];
    let mut output_lowest = vec![0.0; len];

    py.allow_threads(|| {
        willr::willr(
            high_slice,
            low_slice,
            close_slice,
            period,
            output.as_mut_slice(),
            output_highest.as_mut_slice(),
            output_lowest.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output.into_pyarray(py).into(),
        output_highest.into_pyarray(py).into(),
        output_lowest.into_pyarray(py).into(),
    ))
}

/// Incrementally calculates Williams %R for the latest data point.
///
/// This function provides an optimized way to calculate the latest Williams %R value
/// by using previously calculated highest high and lowest low values.
///
/// Args:
///     prev_highest_high: Previous period's highest high value.
///     prev_lowest_low: Previous period's lowest low value.
///     prev_high: Previous period's high price.
///     prev_low: Previous period's low price.
///     close: Current period's closing price.
///     high: Current period's high price.
///     low: Current period's low price.
///
/// Returns:
///     A tuple containing:
///     - Current Williams %R value
///     - New highest high
///     - New lowest low
///
/// Examples:
///     ```python
///     >>> import kand
///     >>> willr, high, low = kand.willr_inc(15.0, 10.0, 14.0, 11.0, 12.0, 13.0, 11.0)
///     ```
#[pyfunction]
#[pyo3(
    name = "willr_inc",
    signature = (prev_highest_high, prev_lowest_low, prev_high, prev_low, close, high, low)
)]
pub fn willr_inc_py(
    prev_highest_high: TAFloat,
    prev_lowest_low: TAFloat,
    prev_high: TAFloat,
    prev_low: TAFloat,
    close: TAFloat,
    high: TAFloat,
    low: TAFloat,
) -> PyResult<(TAFloat, TAFloat, TAFloat)> {
    willr::willr_inc(
        prev_highest_high,
        prev_lowest_low,
        prev_high,
        prev_low,
        close,
        high,
        low,
    )
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
