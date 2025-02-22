use num_traits::{Float, FromPrimitive, ToPrimitive};

use crate::StatsError;

/// Calculates the future value (FV) of an investment with periodic payments and fixed interest rate.
///
/// # Mathematical Formula
/// For non-zero interest rate:
/// ```text
/// FV = -( PV * (1 + r)^n + PMT * (1 + r*w) * ((1 + r)^n - 1) / r )
/// ```
/// For zero interest rate:
/// ```text
/// FV = -( PV + PMT * n )
/// ```
/// Where:
/// - r = interest rate per period
/// - n = number of periods
/// - w = payment timing (0 or 1)
///
/// # Calculation Principle
/// 1. For non-zero rate:
///    - Calculate compound growth factor: (1 + r)^n
///    - Compute present value growth: PV * (1 + r)^n
///    - Calculate payment accumulation with timing adjustment
///    - Combine and negate for final value
/// 2. For zero rate:
///    - Simply sum initial value and total payments
///
/// # Parameters
/// - `rate`: Interest rate per period (e.g. 0.05 for 5%)
/// - `nper`: Number of compounding periods (must be ≥ 0)
/// - `pmt`: Payment per period (negative for outflows)
/// - `pv`: Present value/initial investment
/// - `when`: Payment timing (0 = end of period, 1 = beginning)
///
/// # Returns
/// - `Ok(T)`: Future value (negative for inflows per cash flow convention)
/// - `Err(StatsError)`: Calculation error
///
/// # Errors
/// - `InvalidInput`: If:
///   - nper < 0
///   - when ≠ 0 or 1
///   - Any input is NaN (with deep-check)
/// - `ConversionError`: If when value conversion fails
///
/// # Example
/// ```rust
/// use sigmastatsrs::finance::fv;
///
/// // Calculate 10-year investment with:
/// // - 5% annual rate (monthly)
/// // - $100 monthly payment
/// // - $1000 initial investment
/// // - End-of-month payments
/// let fv = fv(0.05 / 12.0, 120.0, -100.0, -1000.0, 0).unwrap();
/// assert!(fv > 0.0); // Positive return
/// ```
pub fn fv<T, U>(rate: T, nper: T, pmt: T, pv: T, when: U) -> Result<T, StatsError>
where
    T: Float + FromPrimitive,
    U: ToPrimitive,
{
    #[cfg(feature = "check")]
    {
        // Ensure the number of periods is non-negative.
        if nper < T::zero() {
            return Err(StatsError::InvalidInput(
                "nper must be non-negative".to_string(),
            ));
        }
        // Validate that 'when' is either 0 (end-of-period) or 1 (beginning-of-period).
        if let Some(when_val) = when.to_f64() {
            if when_val != 0.0 && when_val != 1.0 {
                return Err(StatsError::InvalidInput(
                    "when must be 0 (end of period) or 1 (beginning of period)".to_string(),
                ));
            }
        }
    }

    #[cfg(feature = "deep-check")]
    {
        // Verify that none of the inputs are NaN.
        if rate.is_nan() || nper.is_nan() || pmt.is_nan() || pv.is_nan() {
            return Err(StatsError::InvalidInput(
                "NaN detected in input parameters".to_string(),
            ));
        }
    }

    // Simplified formula when the interest rate is zero.
    if rate == T::zero() {
        return Ok(-(pv + pmt * nper));
    }

    // Convert the payment timing ('when') to the floating-point type T.
    let when_val = when.to_f64().ok_or_else(|| {
        StatsError::ConversionError("Conversion of 'when' to f64 failed".to_string())
    })?;
    let when_converted = T::from_f64(when_val).ok_or_else(|| {
        StatsError::ConversionError("Conversion from f64 to generic type T failed".to_string())
    })?;

    // Calculate the compound growth factor: (1 + rate)^nper.
    let factor = (T::one() + rate).powf(nper);
    // Adjust the impact of periodic payments: (1 + rate * when) * [(1 + rate)^nper - 1].
    let payment_component = (T::one() + rate * when_converted) * (factor - T::one());

    // Combine the present value and payment components, applying a negation to conform with cash flow conventions.
    let result = -(pv * factor + pmt * payment_component / rate);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Test the future value calculation with standard parameters (end-of-period payments).
    #[test]
    fn test_future_value_standard() {
        let rate = 0.05_f64 / 12.0;
        let nper = 10.0 * 12.0;
        let pmt = -100.0;
        let pv = -100.0;
        let when = 0i32; // payments at end-of-period

        // Expected value computed from:
        // factor = (1 + rate)^nper
        // payment_component = (1 + rate * when) * (factor - 1)
        // fv = -(pv * factor + pmt * (payment_component) / rate)
        let expected = 15692.92889433575_f64;
        let result = fv(rate, nper, pmt, pv, when).unwrap();
        assert_relative_eq!(result, expected, max_relative = 1e-12);
    }

    // Test the scenario when the interest rate is zero.
    #[test]
    fn test_future_value_zero_rate() {
        let rate = 0.0_f64;
        let nper = 10.0;
        let pmt = 100.0;
        let pv = 1000.0;
        let when = 0i32;

        // When rate is zero, the future value reduces to: -(pv + pmt * nper)
        let expected = -(pv + pmt * nper);
        let result = fv(rate, nper, pmt, pv, when).unwrap();
        assert_relative_eq!(result, expected);
    }

    // Test the future value calculation for payments made at the beginning of the period.
    #[test]
    fn test_future_value_beginning_of_period() {
        let rate = 0.1_f64;
        let nper = 5.0;
        let pmt = -100.0;
        let pv = -1000.0;
        let when = 1i32; // payments at beginning-of-period

        // Expected value computed from:
        // factor = (1 + rate)^nper ≈ 1.1^5 ≈ 1.61051
        // payment_component = (1 + rate * when) * (factor - 1) ≈ 1.1 * 0.61051 ≈ 0.671561
        // fv = -(pv * factor + pmt * (payment_component) / rate) ≈ 2282.071
        let expected = 2282.071_f64;
        let result = fv(rate, nper, pmt, pv, when).unwrap();
        assert_relative_eq!(result, expected, max_relative = 1e-6);
    }
}
