use kand::{TAFloat, ohlcv::adosc};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculate Accumulation/Distribution Oscillator (A/D Oscillator or ADOSC)
///
/// The A/D Oscillator is a momentum indicator that measures the difference between a fast and slow EMA of the
/// Accumulation/Distribution Line. It helps identify trend strength and potential reversals.
///
/// Args:
///   high: High prices as a 1-D NumPy array of type `TAFloat`.
///   low: Low prices as a 1-D NumPy array of type `TAFloat`.
///   close: Close prices as a 1-D NumPy array of type `TAFloat`.
///   volume: Volume as a 1-D NumPy array of type `TAFloat`.
///   fast_period: Fast period for A/D Oscillator calculation.
///   slow_period: Slow period for A/D Oscillator calculation.
///
/// Returns:
///   A tuple of 4 1-D NumPy arrays containing:
///   - ADOSC values
///   - A/D Line values
///   - Fast EMA values
///   - Slow EMA values
///   Each array has the same length as the input, with the first `slow_period-1` elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([10.0, 11.0, 12.0, 11.5, 10.5])
///   >>> low = np.array([8.0, 9.0, 10.0, 9.5, 8.5])
///   >>> close = np.array([9.0, 10.0, 11.0, 10.0, 9.0])
///   >>> volume = np.array([100.0, 150.0, 200.0, 150.0, 100.0])
///   >>> adosc, ad, fast_ema, slow_ema = kand.adosc(high, low, close, volume, 3, 5)
///   ```
#[pyfunction]
#[pyo3(name = "adosc", signature = (high, low, close, volume, fast_period, slow_period))]
pub fn adosc_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
    volume: PyReadonlyArray1<TAFloat>,
    fast_period: usize,
    slow_period: usize,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    // Convert input NumPy arrays to Rust slices
    let input_high = high.as_slice()?;
    let input_low = low.as_slice()?;
    let input_close = close.as_slice()?;
    let input_volume = volume.as_slice()?;
    let len = input_high.len();

    // Create output arrays
    let mut output_adosc = vec![0.0; len];
    let mut output_ad = vec![0.0; len];
    let mut output_fast_ema = vec![0.0; len];
    let mut output_slow_ema = vec![0.0; len];

    // Perform ADOSC calculation while releasing the GIL
    py.allow_threads(|| {
        adosc::adosc(
            input_high,
            input_low,
            input_close,
            input_volume,
            fast_period,
            slow_period,
            &mut output_adosc,
            &mut output_ad,
            &mut output_fast_ema,
            &mut output_slow_ema,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert output arrays to Python objects
    Ok((
        output_adosc.into_pyarray(py).into(),
        output_ad.into_pyarray(py).into(),
        output_fast_ema.into_pyarray(py).into(),
        output_slow_ema.into_pyarray(py).into(),
    ))
}

/// Calculate latest A/D Oscillator value incrementally
///
/// Provides optimized calculation of the latest ADOSC value when new data arrives,
/// without recalculating the entire series.
///
/// Args:
///     high: Latest high price.
///     low: Latest low price.
///     close: Latest closing price.
///     volume: Latest volume.
///     prev_ad: Previous A/D value.
///     prev_fast_ema: Previous fast EMA value.
///     prev_slow_ema: Previous slow EMA value.
///     fast_period: Fast EMA period.
///     slow_period: Slow EMA period.
///
/// Returns:
///     A tuple containing (ADOSC, AD, Fast EMA, Slow EMA) values.
///
/// Examples:
///     ```python
///     >>> import kand
///     >>> adosc, ad, fast_ema, slow_ema = kand.adosc_incremental(
///     ...     10.5,  # high
///     ...     9.5,   # low
///     ...     10.0,  # close
///     ...     150.0, # volume
///     ...     100.0, # prev_ad
///     ...     95.0,  # prev_fast_ema
///     ...     90.0,  # prev_slow_ema
///     ...     3,     # fast_period
///     ...     10,    # slow_period
///     ... )
///     ```
#[pyfunction]
#[pyo3(name = "adosc_incremental", signature = (high, low, close, volume, prev_ad, prev_fast_ema, prev_slow_ema, fast_period, slow_period))]
pub fn adosc_incremental_py(
    py: Python,
    high: TAFloat,
    low: TAFloat,
    close: TAFloat,
    volume: TAFloat,
    prev_ad: TAFloat,
    prev_fast_ema: TAFloat,
    prev_slow_ema: TAFloat,
    fast_period: usize,
    slow_period: usize,
) -> PyResult<(TAFloat, TAFloat, TAFloat, TAFloat)> {
    // Perform the incremental ADOSC calculation while releasing the GIL
    py.allow_threads(|| {
        adosc::adosc_incremental(
            high,
            low,
            close,
            volume,
            prev_ad,
            prev_fast_ema,
            prev_slow_ema,
            fast_period,
            slow_period,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
