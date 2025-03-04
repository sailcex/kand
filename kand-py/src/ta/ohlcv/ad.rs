use kand::{TAFloat, ohlcv::ad};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Accumulation/Distribution (A/D) indicator over NumPy arrays.
///
/// The A/D indicator measures the cumulative flow of money into and out of a security by
/// combining price and volume data. It helps identify whether buying or selling pressure
/// is dominant.
///
/// Args:
///     high: High prices as a 1-D NumPy array of type `TAFloat`.
///     low: Low prices as a 1-D NumPy array of type `TAFloat`.
///     close: Close prices as a 1-D NumPy array of type `TAFloat`.
///     volume: Volume data as a 1-D NumPy array of type `TAFloat`.
///
/// Returns:
///     A new 1-D NumPy array containing the A/D values. The array has the same length as the inputs.
/// Examples:
///     ```python
///     >>> import numpy as np
///     >>> import kand
///     >>> high = np.array([10.0, 12.0, 15.0])
///     >>> low = np.array([8.0, 9.0, 11.0])
///     >>> close = np.array([9.0, 11.0, 13.0])
///     >>> volume = np.array([100.0, 150.0, 200.0])
///     >>> result = kand.ad(high, low, close, volume)
///     >>> print(result)
///     [-50.0, 25.0, 125.0]
///     ```
#[pyfunction]
#[pyo3(name = "ad", signature = (high, low, close, volume))]
pub fn ad_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
    volume: PyReadonlyArray1<TAFloat>,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    // Convert the input NumPy arrays to Rust slices
    let high_input = high.as_slice()?;
    let low_input = low.as_slice()?;
    let close_input = close.as_slice()?;
    let volume_input = volume.as_slice()?;
    let len = high_input.len();

    // Create a new output array
    let mut output = vec![0.0; len];

    // Perform the A/D calculation while releasing the GIL to allow other Python threads to run
    py.allow_threads(|| {
        ad::ad(
            high_input,
            low_input,
            close_input,
            volume_input,
            &mut output,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output array to a Python object
    Ok(output.into_pyarray(py).into())
}

/// Computes the latest Accumulation/Distribution (A/D) value incrementally.
///
/// This function calculates only the latest A/D value using the previous A/D value,
/// avoiding recalculation of the entire series.
///
/// Args:
///     high: Latest high price.
///     low: Latest low price.
///     close: Latest closing price.
///     volume: Latest volume.
///     prev_ad: Previous A/D value.
///
/// Returns:
///     The latest A/D value.
///
/// Examples:
///     ```python
///     >>> import kand
///     >>> high = 15.0
///     >>> low = 11.0
///     >>> close = 13.0
///     >>> volume = 200.0
///     >>> prev_ad = 25.0
///     >>> result = kand.ad_inc(high, low, close, volume, prev_ad)
///     >>> print(result)
///     125.0
///     ```
#[pyfunction]
#[pyo3(name = "ad_inc", signature = (high, low, close, volume, prev_ad))]
pub fn ad_inc_py(
    py: Python,
    high: TAFloat,
    low: TAFloat,
    close: TAFloat,
    volume: TAFloat,
    prev_ad: TAFloat,
) -> PyResult<TAFloat> {
    // Perform the incremental A/D calculation while releasing the GIL
    py.allow_threads(|| ad::ad_inc(high, low, close, volume, prev_ad))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
