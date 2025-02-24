use num_traits::{Float, FromPrimitive};

use crate::{KandError, helper::get_levels_hl};

/// Calculates Time Price Opportunity (TPO) profile from OHLCV data.
///
/// # Description
/// TPO shows the distribution of time spent at different price levels.
/// For each price level, it counts how many times the price has visited that level.
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `param_interval` - Size of each price level interval
/// * `output_tpo` - Array to store TPO profile counts
/// * `output_level_ceiling` - Array to store upper price bounds for each level
/// * `output_level_floor` - Array to store lower price bounds for each level
///
/// # Returns
/// * `Result<(), KandError>` - Empty result on success
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input arrays have different lengths
/// * `KandError::InvalidParameter` - If `param_interval` is <= 0
/// * `KandError::InsufficientData` - If output arrays are too small
///
/// # Examples
/// ```
/// use kand::ohlcv::tpo::tpo;
///
/// let highs = vec![100.3, 100.4, 100.2];
/// let lows = vec![100.1, 100.2, 100.0];
/// let interval = 0.1;
///
/// let mut tpo_counts = vec![0.0; 5];
/// let mut level_ceiling = vec![0.0; 5];
/// let mut level_floor = vec![0.0; 5];
///
/// tpo(
///     &highs,
///     &lows,
///     interval,
///     &mut tpo_counts,
///     &mut level_ceiling,
///     &mut level_floor,
/// )
/// .unwrap();
/// ```
pub fn tpo<T>(
    input_high: &[T],
    input_low: &[T],
    param_interval: T,
    output_tpo: &mut [T],
    output_level_ceiling: &mut [T],
    output_level_floor: &mut [T],
) -> Result<(), KandError>
where
    T: Float + FromPrimitive,
{
    let len = input_high.len();
    let (levels, lower_bound, _) = get_levels_hl(input_high, input_low, param_interval)?;

    #[cfg(feature = "check")]
    {
        // Empty data check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Length consistency check
        if len != input_low.len() {
            return Err(KandError::LengthMismatch);
        }

        // Output array size check
        let output_len = output_tpo.len();
        if output_len < levels {
            return Err(KandError::InsufficientData);
        }

        // Output array length consistency check
        if output_len != output_level_ceiling.len() || output_len != output_level_floor.len() {
            return Err(KandError::LengthMismatch);
        }

        // Parameter check
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

    // Initialize output arrays
    for i in 0..levels {
        output_tpo[i] = T::zero();
        let i_t = T::from(i).ok_or(KandError::ConversionError)?;
        let i_plus_1 = T::from(i + 1).ok_or(KandError::ConversionError)?;
        output_level_ceiling[i] = lower_bound + param_interval * i_plus_1;
        output_level_floor[i] = lower_bound + param_interval * i_t;
    }

    // Extract compensation value
    let compensation = T::from_f64(0.000_001).ok_or(KandError::ConversionError)?;

    // Calculate TPO profile
    for i in 0..len {
        let high = input_high[i];
        let low = input_low[i];

        // Calculate level indices for high and low prices
        let high_level = ((high - lower_bound) / param_interval + compensation)
            .floor()
            .to_usize()
            .ok_or(KandError::ConversionError)?;
        let low_level = ((low - lower_bound) / param_interval + compensation)
            .floor()
            .to_usize()
            .ok_or(KandError::ConversionError)?;

        // Increment TPO count for each level between low and high
        for (level, value) in output_tpo
            .iter_mut()
            .enumerate()
            .take(high_level + 1)
            .skip(low_level)
        {
            if level < levels {
                *value = *value + T::one();
            }
        }
    }

    Ok(())
}

/// Calculates Time Price Opportunity (TPO) incrementally for a single period.
///
/// # Description
/// Updates TPO calculations when new price data arrives, without reprocessing the entire dataset.
///
/// # Arguments
/// * `input_high` - High price of the current period
/// * `input_low` - Low price of the current period
/// * `param_interval` - Size of each price level interval
/// * `param_lower_bound` - Lower bound of the price range
///
/// # Returns
/// * `Result<Vec<(T, T, T, usize)>, KandError>` - Vector of tuples containing for each affected level:
///   - `tpo_count`: Count increment for the level (always 1.0)
///   - `level_ceiling`: Upper price bound for the level
///   - `level_floor`: Lower price bound for the level
///   - level: The level index
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_interval` is <= 0
/// * `KandError::NaNDetected` - If any input value is NaN (when "`deep-check`" is enabled)
pub fn tpo_incremental<T>(
    input_high: T,
    input_low: T,
    param_interval: T,
    param_lower_bound: T,
) -> Result<Vec<(T, T, T, usize)>, KandError>
where
    T: Float + FromPrimitive,
{
    #[cfg(feature = "check")]
    {
        if param_interval <= T::zero() {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        if input_high.is_nan() || input_low.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    let mut result = Vec::new();

    // Extract compensation
    let compensation = T::from_f64(0.000_001).ok_or(KandError::ConversionError)?;

    // Calculate level indices for high and low prices
    let high_level = ((input_high - param_lower_bound) / param_interval + compensation)
        .floor()
        .to_usize()
        .ok_or(KandError::ConversionError)?;
    let low_level = ((input_low - param_lower_bound) / param_interval + compensation)
        .floor()
        .to_usize()
        .ok_or(KandError::ConversionError)?;

    // Create entries for each level between low and high
    for level in low_level..=high_level {
        let level_as_t = T::from(level).ok_or(KandError::ConversionError)?;
        let level_floor = level_as_t * param_interval + param_lower_bound;
        let level_ceiling = level_floor + param_interval;
        result.push((T::one(), level_ceiling, level_floor, level));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_tpo() {
        // Test basic functionality of tpo using example data.
        let highs = vec![100.3, 100.4, 100.2];
        let lows = vec![100.1, 100.2, 100.0];
        let interval = 0.1;
        let expected_levels = 5;
        // Price levels will be:
        // [100.0, 100.1) -> level 0 C
        // [100.1, 100.2) -> level 1 A C
        // [100.2, 100.3) -> level 2 A B C
        // [100.3, 100.4) -> level 3 A B
        // [100.4, 100.5) -> level 4 B
        let mut tpo_counts = vec![0.0; expected_levels];
        let mut level_ceiling = vec![0.0; expected_levels];
        let mut level_floor = vec![0.0; expected_levels];

        // Call the tpo function.
        tpo(
            &highs,
            &lows,
            interval,
            &mut tpo_counts,
            &mut level_ceiling,
            &mut level_floor,
        )
        .unwrap();

        // Expected TPO counts:
        // Bar 0 (high=100.3, low=100.1): increments levels 1 to 3
        // Bar 1 (high=100.4, low=100.2): increments levels 2 to 4
        // Bar 2 (high=100.2, low=100.0): increments levels 0 to 2
        // Total counts: [1, 2, 3, 2, 1]
        let expected_counts = [1.0, 2.0, 3.0, 2.0, 1.0];
        for (&actual, &expected) in tpo_counts.iter().zip(expected_counts.iter()) {
            assert_relative_eq!(actual, expected, epsilon = 1e-6, max_relative = 1e-6);
        }

        // Expected level floors and ceilings assuming lower_bound is the minimum low (100.0):
        // level_floor: [100.0, 100.1, 100.2, 100.3, 100.4]
        // level_ceiling: [100.1, 100.2, 100.3, 100.4, 100.5]
        for i in 0..expected_levels {
            let floor_expected = (i as f64).mul_add(interval, 100.0);
            let ceiling_expected = (i as f64 + 1.0).mul_add(interval, 100.0);
            assert_relative_eq!(
                level_floor[i],
                floor_expected,
                epsilon = 1e-6,
                max_relative = 1e-6
            );
            assert_relative_eq!(
                level_ceiling[i],
                ceiling_expected,
                epsilon = 1e-6,
                max_relative = 1e-6
            );
        }
    }

    #[test]
    fn test_tpo_incremental() {
        // Test basic functionality of tpo_incremental.
        let high = 102.5;
        let low = 102.0;
        let interval = 0.5;
        let lower_bound = 102.0;

        let result = tpo_incremental(high, low, interval, lower_bound).unwrap();
        // Expected: two levels (low_level = 0, high_level = 1)
        assert_eq!(result.len(), 2);

        // Verify level 0 values.
        let (count0, ceiling0, floor0, lvl0) = result[0];
        assert_relative_eq!(count0, 1.0, epsilon = 1e-6);
        assert_relative_eq!(floor0, 102.0, epsilon = 1e-6);
        assert_relative_eq!(ceiling0, 102.5, epsilon = 1e-6);
        assert_eq!(lvl0, 0);

        // Verify level 1 values.
        let (count1, ceiling1, floor1, lvl1) = result[1];
        assert_relative_eq!(count1, 1.0, epsilon = 1e-6);
        assert_relative_eq!(floor1, 102.5, epsilon = 1e-6);
        assert_relative_eq!(ceiling1, 103.0, epsilon = 1e-6);
        assert_eq!(lvl1, 1);
    }
}
