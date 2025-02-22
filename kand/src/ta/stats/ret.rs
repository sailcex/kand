use num_traits::{Float, FromPrimitive};

use crate::StatsError;

/// Computes the per-period performance (return rate) between adjacent prices.
///
/// # Mathematical Formula
/// ```text
/// performance = (current_price - previous_price) / previous_price
/// ```
///
/// # Calculation Principle
/// 1. First element is set to 0 (no previous price exists)
/// 2. For each subsequent price pair:
///    - Calculate price difference
///    - Divide by previous price to get return rate
///
/// # Parameters
/// - `values`: Price time series [T]
///   - Must contain at least 2 elements
///   - Each element represents price at a point in time
/// - `performances`: Output slice [T] for storing return rates
///   - Must have same length as values
///   - First element will be 0
///
/// # Returns
/// - `Ok(())`: Calculation successful, results stored in performances slice
/// - `Err(StatsError)`: Calculation failed
///
/// # Errors
/// - `InvalidInput`: If:
///   - Input has fewer than 2 elements
///   - Any previous price is zero
///   - Output slice length differs from input
///   - Input contains NaN values (when deep-check enabled)
///
/// # Example
/// ```rust
/// use sigmastatsrs::finance::ret;
///
/// let prices = vec![100.0, 110.0, 150.0];
/// let mut returns = vec![0.0; prices.len()];
/// ret(&prices, &mut returns).unwrap();
///
/// // Returns:
/// // [0.0,           // No previous price
/// //  0.1,           // (110-100)/100
/// //  0.363636...]   // (150-110)/110
/// ```
pub fn ret<T>(values: &[T], performances: &mut [T]) -> Result<(), StatsError>
where T: Float + FromPrimitive {
    #[cfg(feature = "check")]
    {
        // Ensure there are at least two values
        if values.len() < 2 {
            return Err(crate::StatsError::InvalidInput(
                "At least two price values are required".to_string(),
            ));
        }

        // Validate output slice length
        if performances.len() != values.len() {
            return Err(crate::StatsError::InvalidInput(
                "Output slice must have length equal to input length".to_string(),
            ));
        }
    }

    #[cfg(feature = "deep-check")]
    {
        for (i, &value) in values.iter().enumerate() {
            // Check for NaN values
            if value.is_nan() {
                return Err(crate::StatsError::InvalidInput(
                    "NaN detected in input values".to_string(),
                ));
            }

            // Only check for zero values up to second-to-last element
            if i < values.len() - 1 && value == T::zero() {
                return Err(crate::StatsError::InvalidInput(format!(
                    "Price value at index {} is zero, cannot compute return",
                    i
                )));
            }
        }
    }

    // Set first element to 0
    performances[0] = T::zero();

    // Compute returns for each adjacent pair
    for i in 1..values.len() {
        let previous = values[i - 1];
        let current = values[i];
        performances[i] = (current - previous) / previous;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_ret_positive_return() {
        // For a price series with an increase from 100 to 110 to 150:
        // Returns: [0, (110 - 100)/100 = 0.1, (150 - 110)/110 ≈ 0.363636]
        let prices = vec![100.0_f64, 110.0_f64, 150.0_f64];
        let mut results = vec![0.0; prices.len()];
        ret(&prices, &mut results).unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], 0.0);
        assert_relative_eq!(results[1], 0.1_f64, max_relative = 1e-6);
        assert_relative_eq!(results[2], (150.0 - 110.0) / 110.0, max_relative = 1e-6);
    }

    #[test]
    fn test_ret_negative_return() {
        // For a price series with a decrease from 200 to 180 to 150:
        // Returns: [0, (180 - 200)/200 = -0.1, (150 - 180)/180 ≈ -0.166667]
        let prices = vec![200.0_f64, 180.0_f64, 150.0_f64];
        let mut results = vec![0.0; prices.len()];
        ret(&prices, &mut results).unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], 0.0);
        assert_relative_eq!(results[1], (180.0 - 200.0) / 200.0, max_relative = 1e-6);
        assert_relative_eq!(results[2], (150.0 - 180.0) / 180.0, max_relative = 1e-6);
    }

    #[test]
    fn test_ret_invalid_initial() {
        // Test that a zero previous price in a pair produces an error.
        let prices = vec![0.0_f64, 150.0_f64];
        let mut results = vec![0.0; prices.len()];
        let result = ret(&prices, &mut results);
        assert!(result.is_err());
    }

    #[test]
    fn test_ret_insufficient_data() {
        // Test that providing fewer than two elements produces an error.
        let prices = vec![100.0_f64];
        let mut results = vec![0.0; prices.len()];
        let result = ret(&prices, &mut results);
        assert!(result.is_err());
    }

    #[test]
    fn test_ret_wrong_output_length() {
        // Test that providing an output slice of wrong length produces an error
        let prices = vec![100.0_f64, 110.0_f64, 150.0_f64];
        let mut results = vec![0.0; prices.len() - 1]; // Wrong length (too short)
        let result = ret(&prices, &mut results);
        assert!(result.is_err());
    }
}
