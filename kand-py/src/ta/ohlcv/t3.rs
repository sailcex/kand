use kand::{TAFloat, ohlcv::t3};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the T3 (Triple Exponential Moving Average) indicator over a NumPy array.
///
/// T3 is a sophisticated moving average developed by Tim Tillson that reduces lag while maintaining smoothness.
/// It combines six EMAs with optimized weightings to produce a responsive yet smooth indicator.
///
/// Args:
///     data: Input data as a 1-D NumPy array of type `TAFloat`.
///     period: Smoothing period for EMAs (must be >= 2).
///     vfactor: Volume factor controlling smoothing (typically 0-1).
///
/// Returns:
///     A tuple of seven 1-D NumPy arrays containing:
///     - T3 values
///     - EMA1 values
///     - EMA2 values
///     - EMA3 values
///     - EMA4 values
///     - EMA5 values
///     - EMA6 values
///     Each array has the same length as the input, with initial values being NaN.
///
/// Examples:
///     ```python
///     >>> import numpy as np
///     >>> import kand
///     >>> data = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
///     >>> t3, e1, e2, e3, e4, e5, e6 = kand.t3(data, 2, 0.7)
///     ```
#[pyfunction]
#[pyo3(name = "t3", signature = (data, period, vfactor))]
pub fn t3_py(
    py: Python,
    data: PyReadonlyArray1<TAFloat>,
    period: usize,
    vfactor: TAFloat,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    let input = data.as_slice()?;
    let len = input.len();

    let mut output = vec![0.0; len];
    let mut ema1 = vec![0.0; len];
    let mut ema2 = vec![0.0; len];
    let mut ema3 = vec![0.0; len];
    let mut ema4 = vec![0.0; len];
    let mut ema5 = vec![0.0; len];
    let mut ema6 = vec![0.0; len];

    py.allow_threads(|| {
        t3::t3(
            input,
            period,
            vfactor,
            output.as_mut_slice(),
            ema1.as_mut_slice(),
            ema2.as_mut_slice(),
            ema3.as_mut_slice(),
            ema4.as_mut_slice(),
            ema5.as_mut_slice(),
            ema6.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output.into_pyarray(py).into(),
        ema1.into_pyarray(py).into(),
        ema2.into_pyarray(py).into(),
        ema3.into_pyarray(py).into(),
        ema4.into_pyarray(py).into(),
        ema5.into_pyarray(py).into(),
        ema6.into_pyarray(py).into(),
    ))
}

/// Incrementally calculates the next T3 value.
///
/// This function provides an optimized way to update T3 values in real-time by using
/// previously calculated EMA values.
///
/// Args:
///     price: Latest price value to calculate T3 from.
///     prev_ema1: Previous EMA1 value.
///     prev_ema2: Previous EMA2 value.
///     prev_ema3: Previous EMA3 value.
///     prev_ema4: Previous EMA4 value.
///     prev_ema5: Previous EMA5 value.
///     prev_ema6: Previous EMA6 value.
///     period: Smoothing period for EMAs (must be >= 2).
///     vfactor: Volume factor (typically 0-1).
///
/// Returns:
///     A tuple containing:
///     - Latest T3 value
///     - Updated EMA1 value
///     - Updated EMA2 value
///     - Updated EMA3 value
///     - Updated EMA4 value
///     - Updated EMA5 value
///     - Updated EMA6 value
///
/// Examples:
///     ```python
///     >>> import kand
///     >>> t3, e1, e2, e3, e4, e5, e6 = kand.t3_incremental(
///     ...     100.0,  # New price
///     ...     95.0,   # Previous EMA1
///     ...     94.0,   # Previous EMA2
///     ...     93.0,   # Previous EMA3
///     ...     92.0,   # Previous EMA4
///     ...     91.0,   # Previous EMA5
///     ...     90.0,   # Previous EMA6
///     ...     5,      # Period
///     ...     0.7,    # Volume factor
///     ... )
///     ```
#[pyfunction]
#[pyo3(name = "t3_incremental", signature = (price, prev_ema1, prev_ema2, prev_ema3, prev_ema4, prev_ema5, prev_ema6, period, vfactor))]
pub fn t3_incremental_py(
    price: TAFloat,
    prev_ema1: TAFloat,
    prev_ema2: TAFloat,
    prev_ema3: TAFloat,
    prev_ema4: TAFloat,
    prev_ema5: TAFloat,
    prev_ema6: TAFloat,
    period: usize,
    vfactor: TAFloat,
) -> PyResult<(
    TAFloat,
    TAFloat,
    TAFloat,
    TAFloat,
    TAFloat,
    TAFloat,
    TAFloat,
)> {
    t3::t3_incremental(
        price, prev_ema1, prev_ema2, prev_ema3, prev_ema4, prev_ema5, prev_ema6, period, vfactor,
    )
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
