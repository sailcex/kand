use num_traits::{Float, FromPrimitive};

use crate::StatsError;

/// Calculates the optimal betting fraction using the Kelly Criterion formula to maximize long-term growth rate of capital.
///
/// # Mathematical Formula
/// ```text
/// f* = (b * p - (1 - p)) / b
///
/// where:
/// f* = optimal fraction to bet
/// p = probability of winning (0 ≤ p ≤ 1)
/// b = net odds received on win (b > 0)
/// ```
///
/// # Calculation Principle
/// 1. Validates input parameters:
///    - Probability p must be between 0 and 1
///    - Net odds b must be positive
/// 2. Computes optimal fraction using Kelly formula
/// 3. Returns fraction (negative indicates unfavorable bet)
///
/// # Parameters
/// * `p` - Probability of winning outcome
///   - Must be between 0 and 1 inclusive
///   - Represents estimated win probability
/// * `b` - Net odds received on win
///   - Must be greater than 0
///   - Represents "b-to-1" payout odds
///
/// # Returns
/// * `Ok(T)` - Optimal fraction of bankroll to wager
///   - Positive: Favorable bet, wager indicated fraction
///   - Negative: Unfavorable bet, do not wager
///   - Generic floating point type T
/// * `Err(StatsError)` - Error if inputs are invalid
///
/// # Errors
/// * `StatsError::InvalidInput` if:
///   - p < 0 or p > 1 (with check feature)
///   - b ≤ 0 (with check feature)
///   - p or b is NaN (with deep-check feature)
/// * `StatsError::ConversionError` if type conversion fails
///
/// # Example
/// ```rust
/// use sigmastatsrs::finance::kelly;
///
/// // Calculate optimal bet size for:
/// // - 60% win probability
/// // - 3-to-1 payout odds
/// let fraction = kelly(0.60_f64, 3.0_f64).unwrap();
/// assert!(fraction > 0.0); // Favorable bet
/// ```
pub fn kelly<T>(p: T, b: T) -> Result<T, StatsError>
where T: Float + FromPrimitive {
    #[cfg(feature = "check")]
    {
        // Validate that probability p is within [0, 1].
        if p < T::zero() || p > T::one() {
            return Err(StatsError::InvalidInput(
                "Probability 'p' must be between 0 and 1".to_string(),
            ));
        }
        // Ensure that the net odds b is strictly positive.
        if b <= T::zero() {
            return Err(StatsError::InvalidInput(
                "Odds 'b' must be greater than zero".to_string(),
            ));
        }
    }

    #[cfg(feature = "deep-check")]
    {
        if p.is_nan() || b.is_nan() {
            return Err(StatsError::InvalidInput(
                "NaN detected in input parameters".to_string(),
            ));
        }
    }

    // Compute the Kelly fraction: f* = (b * p - (1 - p)) / b
    let one = T::one();
    let numerator = b * p - (one - p);
    let fraction = numerator / b;
    Ok(fraction)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_kelly_positive_bet() {
        // For p = 0.55 and b = 2:
        // f* = (2*0.55 - 0.45) / 2 = (1.10 - 0.45) / 2 = 0.325.
        let p = 0.55_f64;
        let b = 2.0_f64;
        let result = kelly(p, b).unwrap();
        assert_relative_eq!(result, 0.325_f64, max_relative = 1e-6);
    }

    #[test]
    fn test_kelly_neutral_bet() {
        // For even odds with p = 0.5 and b = 1:
        // f* = (1*0.5 - 0.5) / 1 = 0.
        let p = 0.5_f64;
        let b = 1.0_f64;
        let result = kelly(p, b).unwrap();
        assert_relative_eq!(result, 0.0_f64, max_relative = 1e-6);
    }

    #[test]
    fn test_kelly_negative_bet() {
        // For an unfavorable bet with p = 0.3 and b = 2:
        // f* = (2*0.3 - 0.7) / 2 = (0.6 - 0.7) / 2 = -0.05.
        let p = 0.3_f64;
        let b = 2.0_f64;
        let result = kelly(p, b).unwrap();
        assert_relative_eq!(result, -0.05_f64, max_relative = 1e-6);
    }

    #[test]
    fn test_invalid_probability() {
        // Probability greater than 1 should trigger an error.
        let result = kelly(1.1_f64, 2.0_f64);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_odds() {
        // Non-positive odds should trigger an error.
        let result = kelly(0.5_f64, 0.0_f64);
        assert!(result.is_err());
    }
}
