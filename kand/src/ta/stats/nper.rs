use num_traits::{Float, FromPrimitive, ToPrimitive};

use crate::StatsError;

/// Calculates the number of periods required to reach a target future value with fixed interest rate and payments.
///
/// # Mathematical Formula
/// For non-zero interest rate:
/// ```text
/// nper = ln((pmt * (1 + rate * when) - fv * rate) / (pv * rate + pmt * (1 + rate * when))) / ln(1 + rate)
/// ```
///
/// For zero interest rate:
/// ```text
/// nper = -(fv + pv) / pmt
/// ```
///
/// # Calculation Principle
/// 1. Validates input parameters (when enabled):
///    - Verifies 'when' is 0 or 1
///    - Checks for NaN values
/// 2. Handles zero interest rate case:
///    - Uses simplified linear formula
/// 3. For non-zero rate:
///    - Computes scaling factor (1 + rate * when)
///    - Calculates numerator and denominator
///    - Validates denominator is non-zero
///    - Computes final result using logarithms
///
/// # Parameters
/// * `rate` - Interest rate per period
/// * `pmt` - Payment amount per period
/// * `pv` - Present value or initial investment
/// * `fv` - Target future value
/// * `when` - Payment timing (0 = end of period, 1 = beginning of period)
///
/// # Returns
/// * `Result<T, StatsError>` - Number of periods on success, or error on failure
///
/// # Errors
/// * `StatsError::InvalidInput` if:
///   - 'when' is not 0 or 1
///   - Division by zero occurs
///   - Logarithm argument is non-positive
///   - Zero rate with zero payment
/// * `StatsError::ConversionError` if type conversion fails
///
/// # Example
/// ```rust
/// use sigmastatsrs::finance::nper;
///
/// // Calculate periods needed to pay off $8000 loan
/// // with $150 monthly payments at 7% annual interest
/// let rate = 0.07 / 12.0; // Monthly rate
/// let pmt = -150.0; // Monthly payment
/// let pv = 8000.0; // Loan amount
/// let fv = 0.0; // Target balance
/// let when = 0; // End-of-month payments
///
/// let periods = nper(rate, pmt, pv, fv, when).unwrap();
/// ```
pub fn nper<T, U>(rate: T, pmt: T, pv: T, fv: T, when: U) -> Result<T, StatsError>
where
    T: Float + FromPrimitive,
    U: ToPrimitive,
{
    #[cfg(feature = "check")]
    {
        if let Some(when_val) = when.to_f64() {
            if when_val != 0.0 && when_val != 1.0 {
                return Err(StatsError::InvalidInput(
                    "when must be 0 (end of period) or 1 (beginning of period)".to_string(),
                ));
            }
        } else {
            return Err(StatsError::ConversionError(
                "Conversion of 'when' to f64 failed".to_string(),
            ));
        }
    }

    #[cfg(feature = "deep-check")]
    {
        if rate.is_nan() || pmt.is_nan() || pv.is_nan() || fv.is_nan() {
            return Err(StatsError::InvalidInput(
                "NaN detected in input parameters".to_string(),
            ));
        }
    }

    let one = T::one();

    // Handle the zero interest rate case.
    if rate == T::zero() {
        if pmt == T::zero() {
            return Err(StatsError::InvalidInput(
                "pmt cannot be zero when rate is zero".to_string(),
            ));
        }
        // For zero rate, the equation simplifies to: fv + pv + pmt * nper = 0
        // Solve for nper: nper = -(fv + pv) / pmt
        return Ok(-(fv + pv) / pmt);
    }

    // Convert 'when' to the generic floating point type T.
    let when_val = when.to_f64().ok_or_else(|| {
        StatsError::ConversionError("Conversion of 'when' to f64 failed".to_string())
    })?;
    let when_converted = T::from_f64(when_val).ok_or_else(|| {
        StatsError::ConversionError("Conversion from f64 to generic type T failed".to_string())
    })?;

    // Compute the common factor: (1 + rate * when)
    let factor = one + rate * when_converted;

    // Calculate the numerator and denominator for the logarithm argument:
    // numerator = pmt * factor - fv * rate
    // denominator = pv * rate + pmt * factor
    let numerator = pmt * factor - fv * rate;
    let denominator = pv * rate + pmt * factor;

    // Ensure the denominator is not zero.
    if denominator == T::zero() {
        return Err(StatsError::InvalidInput(
            "Invalid input: division by zero encountered in logarithm argument".to_string(),
        ));
    }

    let ratio = numerator / denominator;
    if ratio <= T::zero() {
        return Err(StatsError::InvalidInput(
            "Invalid input: logarithm argument is non-positive".to_string(),
        ));
    }

    // Compute the number of periods as the ratio of logarithms.
    let nper = ratio.ln() / (one + rate).ln();
    Ok(nper)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Test with non-zero interest rate and payments at end-of-period.
    #[test]
    fn test_nper_nonzero_rate_end_period() {
        // Parameters:
        //   rate   = 0.05 (per period)
        //   when   = 0 (end-of-period)
        //   pv     = 1000.0
        //   pmt    = -100.0
        //   fv     = 0.0
        //
        // Calculation:
        //   factor      = 1 + rate * when = 1.0
        //   numerator   = pmt * factor - fv * rate = -100.0
        //   denominator = pv * rate + pmt * factor = (1000*0.05) - 100 = 50 - 100 = -50
        //   ratio       = (-100) / (-50) = 2.0
        //   nper        = ln(2) / ln(1.05)
        let rate = 0.05_f64;
        let when = 0i32;
        let pv = 1000.0_f64;
        let pmt = -100.0_f64;
        let fv = 0.0_f64;
        let expected = (2_f64.ln()) / (1.05_f64.ln());
        let result = nper(rate, pmt, pv, fv, when).unwrap();
        assert_relative_eq!(result, expected, max_relative = 1e-6);
    }

    // Test with non-zero interest rate and payments at beginning-of-period.
    #[test]
    fn test_nper_nonzero_rate_begin_period() {
        // Parameters:
        //   rate   = 0.05 (per period)
        //   when   = 1 (beginning-of-period)
        //   pv     = 1000.0
        //   pmt    = -100.0
        //   fv     = 0.0
        //
        // Calculation:
        //   factor      = 1 + rate * when = 1.05
        //   numerator   = pmt * factor - fv * rate = -105.0
        //   denominator = pv * rate + pmt * factor = (1000*0.05) - 105 = 50 - 105 = -55.0
        //   ratio       = (-105) / (-55) ≈ 1.90909
        //   nper        = ln(1.90909) / ln(1.05)
        let rate = 0.05_f64;
        let when = 1i32;
        let pv = 1000.0_f64;
        let pmt = -100.0_f64;
        let fv = 0.0_f64;
        let factor = 1.0 + rate * 1.0;
        let numerator = pmt * factor - fv * rate;
        let denominator = pv * rate + pmt * factor;
        let ratio = numerator / denominator;
        let expected = (ratio.ln()) / ((1.0 + rate).ln());
        let result = nper(rate, pmt, pv, fv, when).unwrap();
        assert_relative_eq!(result, expected, max_relative = 1e-6);
    }

    // Test for zero interest rate.
    #[test]
    fn test_nper_zero_rate() {
        // For zero rate, the formula simplifies to:
        //   nper = -(fv + pv) / pmt.
        // Example:
        //   pv  = 1000.0, pmt = -100.0, fv = 0.0  => nper = -(0 + 1000.0) / (-100.0) = 10.
        let rate = 0.0_f64;
        let when = 0i32; // Timing is irrelevant when rate is zero.
        let pv = 1000.0_f64;
        let pmt = -100.0_f64;
        let fv = 0.0_f64;
        let expected = -(fv + pv) / pmt;
        let result = nper(rate, pmt, pv, fv, when).unwrap();
        assert_relative_eq!(result, expected, max_relative = 1e-12);
    }

    // Test error when pmt is zero in the zero interest rate scenario.
    #[test]
    fn test_nper_zero_rate_with_zero_pmt_error() {
        let rate = 0.0_f64;
        let when = 0i32;
        let pv = 1000.0_f64;
        let pmt = 0.0_f64; // Invalid: pmt cannot be zero when rate is zero.
        let fv = 0.0_f64;
        let result = nper(rate, pmt, pv, fv, when);
        assert!(result.is_err());
    }

    // Test error when the denominator for the logarithm argument is zero.
    #[test]
    fn test_nper_denominator_zero_error() {
        // Choose parameters so that:
        //   denominator = pv * rate + pmt * (1 + rate * when) = 0.
        // For rate = 0.05, when = 0 => denominator = 1000*0.05 + pmt = 50 + pmt.
        // Setting pv = 1000, if we choose pmt = -50 then denominator = 50 - 50 = 0.
        let rate = 0.05_f64;
        let when = 0i32;
        let pv = 1000.0_f64;
        let pmt = -50.0_f64;
        let fv = 100.0_f64; // Arbitrary choice for fv.
        let result = nper(rate, pmt, pv, fv, when);
        assert!(result.is_err());
    }

    // Test error when the logarithm argument becomes non-positive.
    #[test]
    fn test_nper_non_positive_ln_argument_error() {
        // Force the numerator to be zero so that the ratio is 0 (which is non-positive).
        // For when = 0:
        //   numerator = pmt - fv * rate.
        // To have numerator = 0, set fv = pmt / rate.
        let rate = 0.05_f64;
        let when = 0i32;
        let pv = 1000.0_f64;
        let pmt = -100.0_f64;
        let fv = pmt / rate; // fv = -2000.0 makes numerator zero.
        let result = nper(rate, pmt, pv, fv, when);
        assert!(result.is_err());
    }
}
