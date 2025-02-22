use pyo3::prelude::*;

/// Calculate Future Value (FV) for an investment.
///
/// Computes the future value (FV) of an investment with regular periodic payments at a fixed interest rate.
///
/// Args:
///   rate (float): The interest rate per period.
///   nper (float): The total number of compounding periods (must be non-negative).
///   pmt (float): The payment made each period (typically negative for cash outflows).
///   pv (float): The present value or initial investment.
///   when (int): Payment timing indicator; 0 for end-of-period, 1 for beginning-of-period.
///
/// Returns:
///   float: The computed future value (expressed as a negative number to conform with cash flow conventions).
///
/// Raises:
///   ValueError: If the input is invalid or a conversion error occurs.
///
/// Example:
///   >>> fv(0.05/12, 120, -100, -1000, 0)
///   15692.92889433575
#[pyfunction]
#[pyo3(name = "fv", signature = (rate, nper, pmt, pv, when))]
pub fn fv_py(py: Python, rate: f64, nper: f64, pmt: f64, pv: f64, when: i32) -> PyResult<f64> {
    py.allow_threads(|| sigmastatsrs::finance::fv(rate, nper, pmt, pv, when))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
