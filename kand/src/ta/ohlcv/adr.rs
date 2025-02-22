use num_traits::{Float, FromPrimitive};

use crate::{ta::ohlcv::sma, KandError};

/// Returns the lookback period required for Average Daily Range (ADR) calculation.
///
/// # Description
/// The lookback period represents the number of data points needed before the first valid output
/// can be calculated. For ADR, this equals the specified period minus one.
///
/// # Arguments
/// * `param_period` - The time period used for ADR calculation (must be >= 2)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period (period - 1) on success
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if `param_period` is less than 2
///
/// # Example
/// ```
/// use kand::ohlcv::adr;
/// let period = 14;
/// let lookback = adr::lookback(period).unwrap();
/// assert_eq!(lookback, 13); // lookback is period - 1
/// ```
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    sma::lookback(param_period)
}

/// Calculates Average Daily Range (ADR) for an entire price series.
///
/// # Description
/// The Average Daily Range measures the average price range over a specified period,
/// helping to identify volatility levels in the market.
///
/// # Mathematical Formula
/// ```text
/// Daily Range = High - Low
/// ADR = SMA(Daily Range, period)
/// ```
///
/// # Calculation Steps
/// 1. Calculate daily range for each period: High - Low
/// 2. Apply Simple Moving Average (SMA) to the daily ranges
/// 3. First (period-1) values will be NaN as they require full period data
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `param_period` - The time period for ADR calculation (must be >= 2)
/// * `output_adr` - Array to store calculated ADR values
///
/// # Returns
/// * `Result<(), KandError>` - Ok(()) on success, Err on failure
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input arrays have different lengths
/// * `KandError::InvalidParameter` - If `param_period` is less than 2
///
/// # Example
/// ```
/// use kand::ohlcv::adr;
/// let input_high = vec![10.0, 12.0, 15.0, 14.0, 13.0];
/// let input_low = vec![8.0, 9.0, 11.0, 10.0, 9.0];
/// let param_period = 3;
/// let mut output_adr = vec![0.0; 5];
///
/// adr::adr(&input_high, &input_low, param_period, &mut output_adr).unwrap();
/// // First two values are NaN (period-1), followed by calculated ADR values
/// ```
pub fn adr<T>(
    input_high: &[T],
    input_low: &[T],
    param_period: usize,
    output_adr: &mut [T],
) -> Result<(), KandError>
where
    T: Float + FromPrimitive,
{
    let len = input_high.len();
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
        if len != input_low.len() || len != output_adr.len() {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        for i in lookback..len {
            // NaN check
            if input_high[i].is_nan() || input_low[i].is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Calculate daily ranges
    let mut ranges = vec![T::zero(); len];
    for i in 0..len {
        ranges[i] = input_high[i] - input_low[i];
    }

    // Calculate SMA of ranges
    sma::sma(&ranges, param_period, output_adr)?;

    Ok(())
}

/// Calculates the next Average Daily Range (ADR) value incrementally.
///
/// # Description
/// Optimizes ADR calculation by using previous ADR value and only calculating
/// the latest value, avoiding recalculation of the entire series.
///
/// # Arguments
/// * `input_prev_adr` - Previous ADR value
/// * `input_new_high` - Latest high price
/// * `input_new_low` - Latest low price
/// * `input_old_high` - Oldest high price to be removed from period
/// * `input_old_low` - Oldest low price to be removed from period
/// * `param_period` - The time period for ADR calculation (must be >= 2)
///
/// # Returns
/// * `Result<T, KandError>` - Latest ADR value on success, Err on failure
///
/// # Errors
/// * `KandError::NaNDetected` - If any input value is NaN
/// * `KandError::InvalidParameter` - If `param_period` is less than 2
///
/// # Example
/// ```
/// use kand::ohlcv::adr;
/// let prev_adr = 3.0;
/// let new_high = 15.0;
/// let new_low = 12.0;
/// let old_high = 10.0;
/// let old_low = 8.0;
/// let period = 14;
///
/// let next_adr =
///     adr::adr_incremental(prev_adr, new_high, new_low, old_high, old_low, period).unwrap();
/// ```
pub fn adr_incremental<T>(
    input_prev_adr: T,
    input_new_high: T,
    input_new_low: T,
    input_old_high: T,
    input_old_low: T,
    param_period: usize,
) -> Result<T, KandError>
where
    T: Float + FromPrimitive,
{
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        if input_prev_adr.is_nan()
            || input_new_high.is_nan()
            || input_new_low.is_nan()
            || input_old_high.is_nan()
            || input_old_low.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let new_range = input_new_high - input_new_low;
    let old_range = input_old_high - input_old_low;

    sma::sma_incremental(input_prev_adr, new_range, old_range, param_period)
}
