use kand::{TAFloat, ta::ohlcv::bop};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculate Balance of Power (BOP) indicator for NumPy arrays.
///
/// The Balance of Power (BOP) is a momentum oscillator that measures the relative strength
/// between buyers and sellers by comparing the closing price to the opening price and
/// normalizing it by the trading range (high - low).
///
/// Args:
///   open: Input opening prices as a 1-D NumPy array of type `TAFloat`.
///   high: Input high prices as a 1-D NumPy array of type `TAFloat`.
///   low: Input low prices as a 1-D NumPy array of type `TAFloat`.
///   close: Input closing prices as a 1-D NumPy array of type `TAFloat`.
///
/// Returns:
///   A 1-D NumPy array containing the BOP values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> open = np.array([10.0, 11.0, 12.0, 13.0])
///   >>> high = np.array([12.0, 13.0, 14.0, 15.0])
///   >>> low = np.array([8.0, 9.0, 10.0, 11.0])
///   >>> close = np.array([11.0, 12.0, 13.0, 14.0])
///   >>> bop = kand.bop(open, high, low, close)
///   ```
#[pyfunction]
#[pyo3(name = "bop", signature = (open, high, low, close))]
pub fn bop_py(
    py: Python,
    open: PyReadonlyArray1<TAFloat>,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    let open_slice = open.as_slice()?;
    let high_slice = high.as_slice()?;
    let low_slice = low.as_slice()?;
    let close_slice = close.as_slice()?;
    let len = open_slice.len();
    let mut output = vec![0.0; len];

    py.allow_threads(|| {
        bop::bop(
            open_slice,
            high_slice,
            low_slice,
            close_slice,
            output.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok(output.into_pyarray(py).into())
}

/// Calculate a single Balance of Power (BOP) value for the latest price data.
///
/// Args:
///   open: Current period's opening price
///   high: Current period's high price
///   low: Current period's low price
///   close: Current period's closing price
///
/// Returns:
///   The calculated BOP value
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> bop = kand.bop_inc(10.0, 12.0, 8.0, 11.0)
///   ```
#[pyfunction]
#[pyo3(name = "bop_inc", signature = (open, high, low, close))]
pub fn bop_inc_py(
    py: Python,
    open: TAFloat,
    high: TAFloat,
    low: TAFloat,
    close: TAFloat,
) -> PyResult<TAFloat> {
    py.allow_threads(|| bop::bop_inc(open, high, low, close))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
