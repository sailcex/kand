use crate::{KandError, TAFloat};

/// Calculates the lookback period required for Simple Moving Average (SMA).
///
/// The lookback period represents the minimum number of data points needed before
/// the first valid SMA value can be calculated.
///
/// # Arguments
/// * `param_period` - The time period for SMA calculation (must be >= 2)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period (period - 1) on success
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if `param_period` < 2
///
/// # Example
/// ```
/// use kand::ohlcv::sma;
/// let period = 14;
/// let lookback = sma::lookback(period).unwrap();
/// assert_eq!(lookback, 13); // Lookback is period - 1
/// ```
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(param_period - 1)
}

/// Calculates Simple Moving Average (SMA) for a price series.
///
/// The Simple Moving Average is a widely used technical indicator that smooths price data
/// by calculating the arithmetic mean over a specified period.
///
/// # Mathematical Formula
/// ```text
/// SMA = (P1 + P2 + ... + Pn) / n
/// ```
/// Where:
/// - P1, P2, ..., Pn are the input values in the period
/// - n is the time period
///
/// # Calculation Steps
/// 1. Sum n consecutive prices in the period
/// 2. Divide by n to get the arithmetic mean
/// 3. Move forward one period and repeat
///
/// # Arguments
/// * `input` - Slice of input price values
/// * `param_period` - The time period for SMA calculation (must be >= 2)
/// * `output_sma` - Mutable slice to store calculated SMA values
///
/// # Returns
/// * `Result<(), KandError>` - Unit type on success
///
/// # Errors
/// * Returns `KandError::InvalidData` if input is empty
/// * Returns `KandError::LengthMismatch` if output length != input length
/// * Returns `KandError::InvalidParameter` if `param_period` < 2
/// * Returns `KandError::InsufficientData` if input length < period
/// * Returns `KandError::NaNDetected` if any input price is NaN (with "`deep-check`" feature)
///
/// # Example
/// ```
/// use kand::ohlcv::sma;
/// let prices = vec![2.0, 4.0, 6.0, 8.0, 10.0];
/// let period = 3;
/// let mut sma_values = vec![0.0; 5];
///
/// sma::sma(&prices, period, &mut sma_values).unwrap();
/// // sma_values = [NaN, NaN, 4.0, 6.0, 8.0]
/// ```
pub fn sma(
    input: &[TAFloat],
    param_period: usize,
    output_sma: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input.len();
    let lookback = lookback(param_period)?;

    #[cfg(feature = "check")]
    {
        // Empty data check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Length consistency check
        if output_sma.len() != len {
            return Err(KandError::LengthMismatch);
        }

        // Data sufficiency check
        if len <= lookback {
            return Err(KandError::InsufficientData);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        // NaN check
        for price in input {
            if price.is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    let mut sum = input[0];
    for value in input.iter().take(lookback + 1).skip(1) {
        sum += *value;
    }
    output_sma[lookback] = sum / param_period as TAFloat;

    for i in (lookback + 1)..input.len() {
        sum = sum + input[i] - input[i - param_period];
        output_sma[i] = sum / param_period as TAFloat;
    }

    // Fill initial values with NAN
    for value in output_sma.iter_mut().take(lookback) {
        *value = TAFloat::NAN;
    }

    Ok(())
}

/// Incrementally calculates the next SMA value.
///
/// This function provides an optimized way to update an existing SMA value
/// when new data arrives, without recalculating the entire series.
///
/// # Mathematical Formula
/// ```text
/// Next SMA = Previous SMA + (New Price - Old Price) / n
/// ```
/// Where:
/// - Previous SMA is the last calculated SMA value
/// - New Price is the latest price to include
/// - Old Price is the oldest price to remove
/// - n is the time period
///
/// # Arguments
/// * `prev_sma` - Previous SMA value
/// * `input_new_price` - New price to include in calculation
/// * `input_old_price` - Oldest price to remove from calculation
/// * `param_period` - The time period for SMA calculation (must be >= 2)
///
/// # Returns
/// * `Result<TAFloat, KandError>` - The next SMA value on success
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if `param_period` < 2
/// * Returns `KandError::NaNDetected` if any input is NaN (with "`deep-check`" feature)
///
/// # Example
/// ```
/// use kand::ohlcv::sma;
/// let prev_sma = 4.0;
/// let new_price = 10.0;
/// let old_price = 2.0;
/// let period = 3;
///
/// let next_sma = sma::sma_incremental(prev_sma, new_price, old_price, period).unwrap();
/// assert_eq!(next_sma, 6.666666666666666);
/// ```
pub fn sma_incremental(
    prev_sma: TAFloat,
    input_new_price: TAFloat,
    input_old_price: TAFloat,
    param_period: usize,
) -> Result<TAFloat, KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        if prev_sma.is_nan() || input_new_price.is_nan() || input_old_price.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    Ok(prev_sma + (input_new_price - input_old_price) / param_period as TAFloat)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Basic functionality tests
    #[test]
    fn test_sma_calculation() {
        let input = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0, 35114.5, 35097.2,
            35092.0, 35073.2, 35139.3, 35092.0, 35126.7, 35106.3, 35124.8, 35170.1, 35215.3,
            35154.0, 35216.3, 35211.8,
        ];
        let param_period = 14;
        let mut output_sma = vec![0.0; input.len()];

        sma(&input, param_period, &mut output_sma).unwrap();

        // First 13 values should be NaN
        for value in output_sma.iter().take(13) {
            assert!(value.is_nan());
        }

        // Test first valid value
        let expected_values = [
            35_203.535_714_285_72,
            35194.55,
            35_181.678_571_428_57,
            35_168.007_142_857_15,
            35_156.821_428_571_435,
            35_148.785_714_285_72,
            35_132.357_142_857_145,
            35113.55,
            35_092.171_428_571_43,
            35_078.057_142_857_15,
            35067.85,
            35_061.057_142_857_15,
            35_052.814_285_714_29,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_sma[i + 13], *expected, epsilon = 0.00001);
        }

        // Now test incremental calculation matches regular calculation
        let mut prev_sma = output_sma[13]; // First valid SMA value

        // Test each incremental step
        for i in 14..17 {
            let result =
                sma_incremental(prev_sma, input[i], input[i - param_period], param_period).unwrap();
            assert_relative_eq!(result, output_sma[i], epsilon = 0.00001);
            prev_sma = result;
        }
    }
}
