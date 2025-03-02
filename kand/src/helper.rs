use crate::{KandError, TAFloat};

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
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if:
///   - The input array is empty
///   - `start_idx` is out of bounds
///   - `lookback` is 0
///   - `start_idx` is less than `lookback - 1`
pub fn lowest_bars(
    array: &[TAFloat],
    start_idx: usize,
    lookback: usize,
) -> Result<usize, KandError> {
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
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if:
///   - The input array is empty
///   - `start_idx` is out of bounds
///   - `lookback` is 0
///   - `start_idx` is less than `lookback - 1`
pub fn highest_bars(
    array: &[TAFloat],
    start_idx: usize,
    lookback: usize,
) -> Result<usize, KandError> {
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
/// * `Result<TAFloat, KandError>` - The calculated k factor if successful, error otherwise
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if `period` is 0
pub fn period_to_k(period: usize) -> Result<TAFloat, KandError> {
    if period == 0 {
        return Err(KandError::InvalidParameter);
    }
    Ok(2.0 / ((period + 1) as TAFloat))
}

/// Calculate candlestick real body length
///
/// # Arguments
/// * `open` - Opening price
/// * `close` - Closing price
///
/// # Returns
/// * `TAFloat` - Absolute difference between open and close prices
#[must_use]
pub fn real_body_length(open: TAFloat, close: TAFloat) -> TAFloat {
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
/// * `TAFloat` - Length of upper shadow
#[must_use]
pub fn upper_shadow_length(high: TAFloat, open: TAFloat, close: TAFloat) -> TAFloat {
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
/// * `TAFloat` - Length of lower shadow
#[must_use]
pub fn lower_shadow_length(low: TAFloat, open: TAFloat, close: TAFloat) -> TAFloat {
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
#[must_use]
pub fn has_real_body_gap_up(
    open2: TAFloat,
    close2: TAFloat,
    open1: TAFloat,
    close1: TAFloat,
) -> bool {
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
#[must_use]
pub fn has_real_body_gap_down(
    open2: TAFloat,
    close2: TAFloat,
    open1: TAFloat,
    close1: TAFloat,
) -> bool {
    open2.max(close2) < open1.min(close1)
}
