use kand::{ohlcv::sar, TAFloat};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculates the Parabolic SAR (Stop And Reverse) indicator over NumPy arrays.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   high: Input high prices as a 1-D NumPy array of type `f32`.
///   low: Input low prices as a 1-D NumPy array of type `f32`.
///   acceleration: Initial acceleration factor (e.g. 0.02).
///   maximum: Maximum acceleration factor (e.g. 0.2).
///
/// Returns:
///   A tuple of four 1-D NumPy arrays containing:
///   - SAR values
///   - Trend direction (true=long, false=short)
///   - Acceleration factors
///   - Extreme points
///   Each array has the same length as the input, with the first element containing NaN.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
///   >>> low = np.array([8.0, 9.0, 11.0, 10.0, 9.0])
///   >>> sar, is_long, af, ep = kand.sar(high, low, 0.02, 0.2)
///   ```
#[pyfunction]
#[pyo3(name = "sar", signature = (high, low, acceleration, maximum))]
pub fn sar_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    acceleration: TAFloat,
    maximum: TAFloat,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<bool>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    let input_high = high.as_slice()?;
    let input_low = low.as_slice()?;
    let len = input_high.len();

    let mut output_sar = vec![0.0; len];
    let mut output_is_long = vec![false; len];
    let mut output_af = vec![0.0; len];
    let mut output_ep = vec![0.0; len];

    py.allow_threads(|| {
        sar::sar(
            input_high,
            input_low,
            acceleration,
            maximum,
            output_sar.as_mut_slice(),
            output_is_long.as_mut_slice(),
            output_af.as_mut_slice(),
            output_ep.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output_sar.into_pyarray(py).into(),
        output_is_long.into_pyarray(py).into(),
        output_af.into_pyarray(py).into(),
        output_ep.into_pyarray(py).into(),
    ))
}

/// Incrementally updates the Parabolic SAR with new price data.
///
/// Args:
///   high: Current period's high price.
///   low: Current period's low price.
///   prev_high: Previous period's high price.
///   prev_low: Previous period's low price.
///   prev_sar: Previous period's SAR value.
///   is_long: Current trend direction (true=long, false=short).
///   af: Current acceleration factor.
///   ep: Current extreme point.
///   acceleration: Acceleration factor increment.
///   maximum: Maximum acceleration factor.
///
/// Returns:
///   A tuple containing (SAR value, trend direction, acceleration factor, extreme point).
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> sar, is_long, af, ep = kand.sar_incremental(
///   ...     15.0, 14.0, 14.5, 13.5, 13.0, True, 0.02, 14.5, 0.02, 0.2
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "sar_incremental", signature = (high, low, prev_high, prev_low, prev_sar, is_long, af, ep, acceleration, maximum))]
pub fn sar_incremental_py(
    high: TAFloat,
    low: TAFloat,
    prev_high: TAFloat,
    prev_low: TAFloat,
    prev_sar: TAFloat,
    is_long: bool,
    af: TAFloat,
    ep: TAFloat,
    acceleration: TAFloat,
    maximum: TAFloat,
) -> PyResult<(TAFloat, bool, TAFloat, TAFloat)> {
    sar::sar_incremental(
        high,
        low,
        prev_high,
        prev_low,
        prev_sar,
        is_long,
        af,
        ep,
        acceleration,
        maximum,
    )
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
