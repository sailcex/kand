use num_traits::{Float, FromPrimitive};

use crate::KandError;

/// Find the number of bars back to the lowest value in a lookback period
///
/// # Arguments
/// * `array` - Array of values to analyze
/// * `start_idx` - Starting index for analysis
/// * `lookback` - Number of bars to look back
///
/// # Visual Example
/// ```text
/// Given array: [5, 2, 4, 1, 3], start_idx = 4, lookback = 3
///
/// Lookback window:        [4, 1, 3]
///                          ^  ^  ^
/// Indices:                 2  3  4
/// Values:                  4  1  3
/// i values:                2  1  0
///                             ↑
///                      Lowest value (1)
///
/// Returns: 1 (number of bars back from start_idx to lowest value)
/// ```
///
/// # Returns
/// * `Result<usize, KandError>` - Number of bars back to lowest value if successful, error otherwise
pub fn lowest_bars<T>(array: &[T], start_idx: usize, lookback: usize) -> Result<usize, KandError>
where T: Float + FromPrimitive {
    if array.is_empty() || start_idx >= array.len() || lookback == 0 || start_idx < lookback - 1 {
        return Err(KandError::InvalidParameter);
    }

    let mut lowest = array[start_idx];
    let mut lowest_idx = 0;

    for i in 1..lookback {
        if array[start_idx - i] < lowest {
            lowest = array[start_idx - i];
            lowest_idx = i;
        }
    }
    Ok(lowest_idx)
}

/// Find the number of bars back to the highest value in a lookback period
///
/// # Arguments
/// * `array` - Array of values to analyze
/// * `start_idx` - Starting index for analysis
/// * `lookback` - Number of bars to look back
///
/// # Visual Example
/// ```text
/// Given array: [1, 4, 2, 5, 3], start_idx = 4, lookback = 3
///
/// Lookback window:        [2, 5, 3]
///                          ^  ^  ^
/// Indices:                 2  3  4
/// Values:                  2  5  3
/// i values:                2  1  0
///                             ↑
///                      Highest value (5)
///
/// Returns: 1 (number of bars back from start_idx to highest value)
/// ```
///
/// # Returns
/// * `Result<usize, KandError>` - Number of bars back to highest value if successful, error otherwise
pub fn highest_bars<T>(array: &[T], start_idx: usize, lookback: usize) -> Result<usize, KandError>
where T: Float + FromPrimitive {
    if array.is_empty() || start_idx >= array.len() || lookback == 0 || start_idx < lookback - 1 {
        return Err(KandError::InvalidParameter);
    }

    let mut highest = array[start_idx];
    let mut highest_idx = 0;

    for i in 1..lookback {
        if array[start_idx - i] > highest {
            highest = array[start_idx - i];
            highest_idx = i;
        }
    }
    Ok(highest_idx)
}

/// Calculate k factor from period value (k = 2 / (period + 1))
///
/// # Arguments
/// * `period` - The period value to convert
///
/// # Returns
/// * `Result<T, KandError>` - The calculated k factor if successful, error otherwise
pub fn period_to_k<T>(period: usize) -> Result<T, KandError>
where T: Float + FromPrimitive {
    if period == 0 {
        return Err(KandError::InvalidParameter);
    }
    Ok(T::from(2).unwrap() / T::from(period + 1).ok_or(KandError::ConversionError)?)
}

/// Calculate candlestick real body length
///
/// # Arguments
/// * `open` - Opening price
/// * `close` - Closing price
///
/// # Returns
/// * `T` - Absolute difference between open and close prices
pub fn real_body_length<T>(open: T, close: T) -> T
where T: Float {
    (close - open).abs()
}

/// Calculate candlestick upper shadow length
///
/// # Arguments
/// * `high` - High price
/// * `open` - Opening price
/// * `close` - Closing price
///
/// # Returns
/// * `T` - Length of upper shadow
pub fn upper_shadow_length<T>(high: T, open: T, close: T) -> T
where T: Float {
    high - if close >= open { close } else { open }
}

/// Calculate candlestick lower shadow length
///
/// # Arguments
/// * `low` - Low price
/// * `open` - Opening price
/// * `close` - Closing price
///
/// # Returns
/// * `T` - Length of lower shadow
pub fn lower_shadow_length<T>(low: T, open: T, close: T) -> T
where T: Float {
    if close >= open {
        open - low
    } else {
        close - low
    }
}

/// Check if there is a gap up between real bodies of two candlesticks
///
/// # Arguments
/// * `open2` - Opening price of second candle
/// * `close2` - Closing price of second candle
/// * `open1` - Opening price of first candle
/// * `close1` - Closing price of first candle
///
/// # Returns
/// * `bool` - True if gap up exists, false otherwise
pub fn has_real_body_gap_up<T>(open2: T, close2: T, open1: T, close1: T) -> bool
where T: Float {
    open2.min(close2) > open1.max(close1)
}

/// Check if there is a gap down between real bodies of two candlesticks
///
/// # Arguments
/// * `open2` - Opening price of second candle
/// * `close2` - Closing price of second candle
/// * `open1` - Opening price of first candle
/// * `close1` - Closing price of first candle
///
/// # Returns
/// * `bool` - True if gap down exists, false otherwise
pub fn has_real_body_gap_down<T>(open2: T, close2: T, open1: T, close1: T) -> bool
where T: Float {
    open2.max(close2) < open1.min(close1)
}

/// Returns the number of price levels and bounds needed for TPO calculation from high/low arrays.
///
/// # Description
/// Similar to `get_levels()` but takes two arrays of high and low prices to determine the range.
/// Useful for calculating levels from OHLCV data.
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `param_interval` - Size of each price level interval
///
/// # Returns
/// * `Result<(usize, T, T), KandError>` - A tuple containing:
///   - Number of price levels needed
///   - Lower bound of the price range
///   - Upper bound of the price range
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if `param_interval` is <= 0
/// * Returns `KandError::InvalidData` if input arrays are empty
/// * Returns `KandError::LengthMismatch` if input arrays have different lengths
///
/// # Examples
/// ```
/// use kand::helper::get_levels_hl;
///
/// let highs = vec![25.8, 22.1, 20.3];
/// let lows = vec![10.5, 15.7, 12.4];
/// let interval = 5.0;
///
/// let (levels, lower, upper) = get_levels_hl(&highs, &lows, interval).unwrap();
/// assert_eq!(levels, 4);
/// assert_eq!(lower, 10.0);
/// assert_eq!(upper, 30.0);
/// ```
pub fn get_levels_hl<T>(
    input_high: &[T],
    input_low: &[T],
    param_interval: T,
) -> Result<(usize, T, T), KandError>
where
    T: Float + FromPrimitive,
{
    let len = input_high.len();

    #[cfg(feature = "check")]
    {
        if len == 0 {
            return Err(KandError::InvalidData);
        }
        if len != input_low.len() {
            return Err(KandError::LengthMismatch);
        }
        if param_interval <= T::zero() {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        // Check for NaN values in input arrays
        for i in 0..len {
            if input_high[i].is_nan() || input_low[i].is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Find min and max prices across both arrays
    let mut min_price = input_low[0];
    let mut max_price = input_high[0];
    for i in 1..len {
        if input_low[i] < min_price {
            min_price = input_low[i];
        }
        if input_high[i] > max_price {
            max_price = input_high[i];
        }
    }

    // Calculate bounds and levels using same logic as get_levels()
    let lower_bound = ((min_price / param_interval)
        + T::from_f64(0.000_001).ok_or(KandError::ConversionError)?)
    .floor()
        * param_interval;
    let upper_bound = ((max_price / param_interval)
        + T::from_f64(0.000_001).ok_or(KandError::ConversionError)?)
    .ceil()
        * param_interval;

    let adjusted_upper_bound = if upper_bound == max_price {
        upper_bound + param_interval
    } else {
        upper_bound
    };

    let levels = ((adjusted_upper_bound - lower_bound) / param_interval).round();
    let levels_usize = levels.to_usize().ok_or(KandError::ConversionError)?;

    Ok((levels_usize, lower_bound, adjusted_upper_bound))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_get_levels_hl() {
        // Test case 1: Regular price distribution with high-low ranges
        // Price levels will be:
        // [10.0, 15.0)
        // [15.0, 20.0)
        // [20.0, 25.0)
        // [25.0, 30.0)
        let highs = vec![25.8, 22.1, 20.3];
        let lows = vec![10.5, 15.7, 12.4];
        let interval = 5.0;
        let (levels, lower, upper) = get_levels_hl(&highs, &lows, interval).unwrap();

        assert_eq!(levels, 4);
        assert_relative_eq!(lower, 10.0);
        assert_relative_eq!(upper, 30.0);

        // Test case 2: Tight price cluster with small decimals
        // Price levels will be:
        // [100.10, 100.20)
        // [100.20, 100.30)
        // [100.30, 100.40)
        // [100.40, 100.50)
        let highs = vec![100.4, 100.3, 100.35];
        let lows = vec![100.1, 100.15, 100.2];
        let interval = 0.1;
        let (levels, lower, upper) = get_levels_hl(&highs, &lows, interval).unwrap();

        assert_eq!(levels, 4);
        assert_relative_eq!(lower, 100.1);
        assert_relative_eq!(upper, 100.5);

        // Test case 3: Very small decimal intervals
        // Price levels will be:
        // [100.000, 100.001)
        // [100.001, 100.002)
        // [100.002, 100.003)
        let highs = vec![100.0022, 100.0018, 100.0015];
        let lows = vec![100.0001, 100.0008, 100.0005];
        let interval = 0.001;
        let (levels, lower, upper) = get_levels_hl(&highs, &lows, interval).unwrap();

        assert_eq!(levels, 3);
        assert_relative_eq!(lower, 100.000);
        assert_relative_eq!(upper, 100.003);

        // Test case 4: Error cases
        // Empty arrays
        let result = get_levels_hl::<f64>(&[], &[], 1.0);
        assert!(matches!(result, Err(KandError::InvalidData)));

        // Length mismatch
        let result = get_levels_hl(&[1.0, 2.0], &[1.0], 1.0);
        assert!(matches!(result, Err(KandError::LengthMismatch)));

        // Invalid interval
        let result = get_levels_hl(&[1.0], &[1.0], 0.0);
        assert!(matches!(result, Err(KandError::InvalidParameter)));

        // Test case 5: NaN detection (only with deep_check feature)
        #[cfg(feature = "deep-check")]
        {
            let result = get_levels_hl(&[f64::NAN, 100.0], &[99.0, 98.0], 1.0);
            assert!(matches!(result, Err(KandError::NaNDetected)));

            let result = get_levels_hl(&[100.0, 101.0], &[f64::NAN, 98.0], 1.0);
            assert!(matches!(result, Err(KandError::NaNDetected)));
        }
    }
}
