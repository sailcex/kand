use kand::{TAFloat, ohlcv::vwap};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculates Volume Weighted Average Price (VWAP) for a series of price data.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   high: High prices as a 1-D NumPy array of type `f32`.
///   low: Low prices as a 1-D NumPy array of type `f32`.
///   close: Close prices as a 1-D NumPy array of type `f32`.
///   volume: Volume data as a 1-D NumPy array of type `f32`.
///
/// Returns:
///   A tuple of three 1-D NumPy arrays containing:
///   - VWAP values
///   - Cumulative price-volume products
///   - Cumulative volumes
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([10.0, 12.0, 15.0])
///   >>> low = np.array([8.0, 9.0, 11.0])
///   >>> close = np.array([9.0, 10.0, 12.0])
///   >>> volume = np.array([100.0, 150.0, 200.0])
///   >>> vwap, cum_pv, cum_vol = kand.vwap(high, low, close, volume)
///   ```
#[pyfunction]
#[pyo3(name = "vwap", signature = (high, low, close, volume))]
pub fn vwap_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
    volume: PyReadonlyArray1<TAFloat>,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    let high_slice = high.as_slice()?;
    let low_slice = low.as_slice()?;
    let close_slice = close.as_slice()?;
    let volume_slice = volume.as_slice()?;
    let len = high_slice.len();

    let mut output_vwap = vec![0.0; len];
    let mut output_cum_pv = vec![0.0; len];
    let mut output_cum_vol = vec![0.0; len];

    py.allow_threads(|| {
        vwap::vwap(
            high_slice,
            low_slice,
            close_slice,
            volume_slice,
            output_vwap.as_mut_slice(),
            output_cum_pv.as_mut_slice(),
            output_cum_vol.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output_vwap.into_pyarray(py).into(),
        output_cum_pv.into_pyarray(py).into(),
        output_cum_vol.into_pyarray(py).into(),
    ))
}

/// Calculates a single VWAP value from the latest price and volume data.
///
/// Args:
///   high: Latest high price value as `f32`.
///   low: Latest low price value as `f32`.
///   close: Latest close price value as `f32`.
///   volume: Latest volume value as `f32`.
///   prev_cum_pv: Previous cumulative price-volume product as `f32`.
///   prev_cum_vol: Previous cumulative volume as `f32`.
///
/// Returns:
///   A tuple containing (new cumulative PV, new cumulative volume, new VWAP).
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> new_cum_pv, new_cum_vol, vwap = kand.vwap_incremental(15.0, 11.0, 14.0, 200.0, 1000.0, 150.0)
///   ```
#[pyfunction]
#[pyo3(name = "vwap_incremental", signature = (high, low, close, volume, prev_cum_pv, prev_cum_vol))]
pub fn vwap_incremental_py(
    high: TAFloat,
    low: TAFloat,
    close: TAFloat,
    volume: TAFloat,
    prev_cum_pv: TAFloat,
    prev_cum_vol: TAFloat,
) -> PyResult<(TAFloat, TAFloat, TAFloat)> {
    vwap::vwap_incremental(high, low, close, volume, prev_cum_pv, prev_cum_vol)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
