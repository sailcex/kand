use crate::{KandError, TAFloat};

/// Calculates the lookback period required for RMA calculation.
///
/// Returns the number of data points needed before RMA can start producing valid values.
/// The lookback period equals the period minus 1, since RMA requires a full period of data
/// to calculate the first value.
///
/// # Arguments
/// * `param_period` - The period length used for RMA calculation (must be >= 2)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period on success
///
/// # Errors
/// * `KandError::InvalidParameter` - If period is less than 2
///
/// # Examples
/// ```
/// use kand::ohlcv::rma;
/// let period = 14;
/// let lookback = rma::lookback(period).unwrap();
/// assert_eq!(lookback, 13); // lookback is period - 1
/// ```
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(param_period - 1)
}

/// Calculates the Running Moving Average (RMA) for a price series.
///
/// RMA is a type of moving average that gives more weight to recent prices while still maintaining
/// some influence from all past prices. It is similar to EMA but uses a different smoothing factor.
///
/// # Mathematical Formula
/// ```text
/// RMA = (Current Price * α) + Previous RMA * (1 - α)
/// where α = 1/period
/// ```
///
/// # Calculation Steps
/// 1. Calculate initial SMA value using first `period` prices
/// 2. For remaining values, apply RMA formula using smoothing factor α = 1/period
/// 3. Fill initial values before period with NaN
///
/// # Arguments
/// * `input` - Array of price values to calculate RMA
/// * `param_period` - The smoothing period (must be >= 2)
/// * `output_rma` - Array to store calculated RMA values
///
/// # Returns
/// * `Result<(), KandError>` - Empty result on success
///
/// # Errors
/// * `KandError::InvalidData` - If input array is empty
/// * `KandError::LengthMismatch` - If input and output arrays have different lengths
/// * `KandError::InvalidParameter` - If period is less than 2
/// * `KandError::InsufficientData` - If input length is less than period
/// * `KandError::NaNDetected` - If input contains NaN values (with "`deep-check`" feature)
///
/// # Examples
/// ```
/// use kand::ohlcv::rma;
/// let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let period = 3;
/// let mut rma_values = vec![0.0; 5];
/// rma::rma(&prices, period, &mut rma_values).unwrap();
/// ```
pub fn rma(
    input: &[TAFloat],
    param_period: usize,
    output_rma: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input.len();
    let lookback = lookback(param_period)?;

    #[cfg(feature = "check")]
    {
        // Empty data check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Data sufficiency check
        if len <= lookback {
            return Err(KandError::InsufficientData);
        }

        // Length consistency check
        if len != output_rma.len() {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        for price in input.iter().take(len) {
            // NaN check
            if price.is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Calculate first SMA value
    let mut sum = input[0];
    for value in input.iter().take(param_period).skip(1) {
        sum += *value;
    }
    let alpha = 1.0 / param_period as TAFloat;
    output_rma[param_period - 1] = sum / param_period as TAFloat;

    // Calculate RMA for remaining values
    for i in param_period..input.len() {
        output_rma[i] = input[i] * alpha + output_rma[i - 1] * (1.0 - alpha);
    }

    // Fill initial values with NAN
    for value in output_rma.iter_mut().take(param_period - 1) {
        *value = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates a single new RMA value incrementally.
///
/// This function enables real-time RMA calculation by computing the next value
/// using only the current price and previous RMA, without requiring historical data.
///
/// # Mathematical Formula
/// ```text
/// RMA = (Current Price * α) + Previous RMA * (1 - α)
/// where α = 1/period
/// ```
///
/// # Arguments
/// * `input_current` - The current price value
/// * `prev_rma` - The previous RMA value
/// * `param_period` - The smoothing period (must be >= 2)
///
/// # Returns
/// * `Result<TAFloat, KandError>` - The new RMA value on success
///
/// # Errors
/// * `KandError::InvalidParameter` - If period is less than 2
/// * `KandError::NaNDetected` - If any input is NaN (with "`deep-check`" feature)
///
/// # Examples
/// ```
/// use kand::ohlcv::rma;
/// let current_price = 10.0;
/// let prev_rma = 9.5;
/// let period = 14;
/// let new_rma = rma::rma_incremental(current_price, prev_rma, period).unwrap();
/// ```
pub fn rma_incremental(
    input_current: TAFloat,
    prev_rma: TAFloat,
    param_period: usize,
) -> Result<TAFloat, KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        // NaN check
        if input_current.is_nan() || prev_rma.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    let alpha = 1.0 / param_period as TAFloat;
    Ok(input_current * alpha + prev_rma * (1.0 - alpha))
}
