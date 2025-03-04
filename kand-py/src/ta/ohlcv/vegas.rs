use kand::{TAFloat, ohlcv::vegas};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the VEGAS (Volume and EMA Guided Adaptive Scaling) indicator over NumPy arrays.
///
/// VEGAS is a trend following indicator that uses multiple EMAs to define channels and boundaries.
///
/// Args:
///   prices: Input prices as a 1-D NumPy array of type `TAFloat`.
///
/// Returns:
///   A tuple of four 1-D NumPy arrays containing:
///   - Channel Upper (EMA 144)
///   - Channel Lower (EMA 169)
///   - Boundary Upper (EMA 576)
///   - Boundary Lower (EMA 676)
///   Each array has the same length as the input, with the first 675 elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> prices = np.array([44.34, 44.09, 44.15, 43.61, 44.33])
///   >>> ch_upper, ch_lower, b_upper, b_lower = kand.vegas(prices)
///   ```
#[pyfunction]
#[pyo3(name = "vegas", signature = (prices))]
pub fn vegas_py(
    py: Python,
    prices: PyReadonlyArray1<TAFloat>,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    // Convert the input NumPy array to a Rust slice
    let input_prices = prices.as_slice()?;
    let len = input_prices.len();

    // Create new output arrays using vec
    let mut output_channel_upper = vec![0.0; len];
    let mut output_channel_lower = vec![0.0; len];
    let mut output_boundary_upper = vec![0.0; len];
    let mut output_boundary_lower = vec![0.0; len];

    // Perform the VEGAS calculation while releasing the GIL
    py.allow_threads(|| {
        vegas::vegas(
            input_prices,
            output_channel_upper.as_mut_slice(),
            output_channel_lower.as_mut_slice(),
            output_boundary_upper.as_mut_slice(),
            output_boundary_lower.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output arrays to Python objects
    Ok((
        output_channel_upper.into_pyarray(py).into(),
        output_channel_lower.into_pyarray(py).into(),
        output_boundary_upper.into_pyarray(py).into(),
        output_boundary_lower.into_pyarray(py).into(),
    ))
}

/// Incrementally calculates the next VEGAS values.
///
/// Args:
///   price: Current price value.
///   prev_channel_upper: Previous EMA(144) value.
///   prev_channel_lower: Previous EMA(169) value.
///   prev_boundary_upper: Previous EMA(576) value.
///   prev_boundary_lower: Previous EMA(676) value.
///
/// Returns:
///   A tuple containing:
///   - Updated Channel Upper value
///   - Updated Channel Lower value
///   - Updated Boundary Upper value
///   - Updated Boundary Lower value
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> price = 100.0
///   >>> prev_values = (98.0, 97.5, 96.0, 95.5)
///   >>> ch_upper, ch_lower, b_upper, b_lower = kand.vegas_inc(
///   ...     price,
///   ...     prev_values[0],
///   ...     prev_values[1],
///   ...     prev_values[2],
///   ...     prev_values[3]
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "vegas_inc", signature = (price, prev_channel_upper, prev_channel_lower, prev_boundary_upper, prev_boundary_lower))]
pub fn vegas_inc_py(
    price: TAFloat,
    prev_channel_upper: TAFloat,
    prev_channel_lower: TAFloat,
    prev_boundary_upper: TAFloat,
    prev_boundary_lower: TAFloat,
) -> PyResult<(TAFloat, TAFloat, TAFloat, TAFloat)> {
    vegas::vegas_inc(
        price,
        prev_channel_upper,
        prev_channel_lower,
        prev_boundary_upper,
        prev_boundary_lower,
    )
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
