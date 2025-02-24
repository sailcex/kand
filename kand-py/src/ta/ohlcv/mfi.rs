use kand::{TAFloat, ohlcv::mfi};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculates the Money Flow Index (MFI) for a NumPy array.
///
/// The Money Flow Index (MFI) is a technical oscillator that uses price and volume data to identify
/// overbought or oversold conditions in an asset.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   high: Array of high prices as a 1-D NumPy array of type `f32`.
///   low: Array of low prices as a 1-D NumPy array of type `f32`.
///   close: Array of close prices as a 1-D NumPy array of type `f32`.
///   volume: Array of volume data as a 1-D NumPy array of type `f32`.
///   period: The time period for MFI calculation (typically 14).
///
/// Returns:
///   A tuple of five 1-D NumPy arrays containing:
///   - MFI values (0-100)
///   - Typical prices
///   - Money flows
///   - Positive money flows
///   - Negative money flows
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([10.0, 11.0, 12.0, 11.0])
///   >>> low = np.array([8.0, 9.0, 10.0, 9.0])
///   >>> close = np.array([9.0, 10.0, 11.0, 10.0])
///   >>> volume = np.array([100.0, 150.0, 200.0, 150.0])
///   >>> mfi, typ_prices, money_flows, pos_flows, neg_flows = kand.mfi(high, low, close, volume, 2)
///   ```
#[pyfunction]
#[pyo3(name = "mfi", signature = (high, low, close, volume, period))]
pub fn mfi_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
    volume: PyReadonlyArray1<TAFloat>,
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
    let volume_slice = volume.as_slice()?;
    let len = high_slice.len();

    let mut mfi = vec![0.0; len];
    let mut typ_prices = vec![0.0; len];
    let mut money_flows = vec![0.0; len];
    let mut pos_flows = vec![0.0; len];
    let mut neg_flows = vec![0.0; len];

    py.allow_threads(|| {
        mfi::mfi(
            high_slice,
            low_slice,
            close_slice,
            volume_slice,
            period,
            mfi.as_mut_slice(),
            typ_prices.as_mut_slice(),
            money_flows.as_mut_slice(),
            pos_flows.as_mut_slice(),
            neg_flows.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        mfi.into_pyarray(py).into(),
        typ_prices.into_pyarray(py).into(),
        money_flows.into_pyarray(py).into(),
        pos_flows.into_pyarray(py).into(),
        neg_flows.into_pyarray(py).into(),
    ))
}
