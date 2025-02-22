use kand::{ohlcv::ecl, TAFloat};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Expanded Camarilla Levels (ECL) over NumPy arrays.
///
/// The ECL indicator calculates multiple support and resistance levels based on the previous period's
/// high, low and close prices.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   high: Input high prices as a 1-D NumPy array of type `f32`.
///   low: Input low prices as a 1-D NumPy array of type `f32`.
///   close: Input close prices as a 1-D NumPy array of type `f32`.
///
/// Returns:
///   A tuple of ten 1-D NumPy arrays containing the ECL values (H5,H4,H3,H2,H1,L1,L2,L3,L4,L5).
///   Each array has the same length as the input, with the first element containing NaN value.
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
///   >>> h5,h4,h3,h2,h1,l1,l2,l3,l4,l5 = kand.ecl(high, low, close)
///   ```
#[pyfunction]
#[pyo3(name = "ecl", signature = (high, low, close))]
pub fn ecl_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    let input_high = high.as_slice()?;
    let input_low = low.as_slice()?;
    let input_close = close.as_slice()?;
    let len = input_high.len();

    let mut output_h5 = vec![0.0; len];
    let mut output_h4 = vec![0.0; len];
    let mut output_h3 = vec![0.0; len];
    let mut output_h2 = vec![0.0; len];
    let mut output_h1 = vec![0.0; len];
    let mut output_l1 = vec![0.0; len];
    let mut output_l2 = vec![0.0; len];
    let mut output_l3 = vec![0.0; len];
    let mut output_l4 = vec![0.0; len];
    let mut output_l5 = vec![0.0; len];

    py.allow_threads(|| {
        ecl::ecl(
            input_high,
            input_low,
            input_close,
            output_h5.as_mut_slice(),
            output_h4.as_mut_slice(),
            output_h3.as_mut_slice(),
            output_h2.as_mut_slice(),
            output_h1.as_mut_slice(),
            output_l1.as_mut_slice(),
            output_l2.as_mut_slice(),
            output_l3.as_mut_slice(),
            output_l4.as_mut_slice(),
            output_l5.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output_h5.into_pyarray(py).into(),
        output_h4.into_pyarray(py).into(),
        output_h3.into_pyarray(py).into(),
        output_h2.into_pyarray(py).into(),
        output_h1.into_pyarray(py).into(),
        output_l1.into_pyarray(py).into(),
        output_l2.into_pyarray(py).into(),
        output_l3.into_pyarray(py).into(),
        output_l4.into_pyarray(py).into(),
        output_l5.into_pyarray(py).into(),
    ))
}

/// Computes the latest Expanded Camarilla Levels (ECL) values incrementally.
///
/// This function provides an efficient way to calculate ECL values for new data without
/// reprocessing the entire dataset.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   prev_high: Previous period's high price as `f32`.
///   prev_low: Previous period's low price as `f32`.
///   prev_close: Previous period's close price as `f32`.
///
/// Returns:
///   A tuple of ten values (H5,H4,H3,H2,H1,L1,L2,L3,L4,L5) containing the latest ECL levels.
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> h5,h4,h3,h2,h1,l1,l2,l3,l4,l5 = kand.ecl_incremental(24.20, 23.85, 23.89)
///   ```
#[pyfunction]
#[pyo3(name = "ecl_incremental", signature = (prev_high, prev_low, prev_close))]
pub fn ecl_incremental_py(
    py: Python,
    prev_high: TAFloat,
    prev_low: TAFloat,
    prev_close: TAFloat,
) -> PyResult<(
    TAFloat,
    TAFloat,
    TAFloat,
    TAFloat,
    TAFloat,
    TAFloat,
    TAFloat,
    TAFloat,
    TAFloat,
    TAFloat,
)> {
    py.allow_threads(|| {
        ecl::ecl_incremental(prev_high, prev_low, prev_close)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    })
}
