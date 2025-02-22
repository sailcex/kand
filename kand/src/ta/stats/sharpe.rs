use num_traits::{Float, FromPrimitive};

use crate::StatsError;

/// Calculates the Sharpe ratio to measure risk-adjusted investment returns by comparing
/// excess returns to volatility.
///
/// # Mathematical Formula
/// The Sharpe ratio (SR) is calculated as:
///
/// SR = (R - Rf) / σ
///
/// where:
/// - R: Mean return of the investment
/// - Rf: Risk-free rate
/// - σ: Standard deviation of returns (volatility)
///
/// # Calculation Steps
/// 1. Calculate the arithmetic mean of returns (R)
/// 2. Subtract risk-free rate (Rf) to get excess returns
/// 3. Calculate standard deviation of returns (σ)
/// 4. Divide excess returns by standard deviation
///
/// # Parameters
/// - `returns`: Slice of periodic investment returns as floating point numbers
///   - Must contain at least 2 data points for valid calculation
///   - Each value represents a return for one period
/// - `risk_free_rate`: Risk-free rate for the same period as returns
///   - Typically treasury bill or similar low-risk asset return
///
/// # Returns
/// - `Ok(T)`: Computed Sharpe ratio as a generic floating point number
///   - Higher values indicate better risk-adjusted returns
/// - `Err(StatsError)`: Error with detailed message if calculation fails
///
/// # Errors
/// - `StatsError::InvalidInput`:
///   - Input slice has fewer than 2 elements
///   - Standard deviation is zero (undefined ratio)
///   - Input contains NaN values (when deep-check enabled)
/// - `StatsError::ConversionError`:
///   - Failed to convert between numeric types
///
/// # Example
/// ```rust
/// use sigmastatsrs::finance::sharpe;
///
/// // Monthly returns data
/// let returns = vec![0.1, 0.2, 0.15, 0.05, 0.1];
/// let risk_free_rate = 0.05; // 5% risk-free rate
///
/// let sharpe_ratio = sharpe(&returns, risk_free_rate).unwrap();
/// ```
pub fn sharpe<T>(returns: &[T], risk_free_rate: T) -> Result<T, StatsError>
where T: Float + FromPrimitive {
    #[cfg(feature = "check")]
    {
        // Ensure there are at least two return observations.
        if returns.len() < 2 {
            return Err(StatsError::InvalidInput(
                "At least two return values are required to compute the Sharpe ratio".to_string(),
            ));
        }
    }

    #[cfg(feature = "deep-check")]
    {
        // Check for NaN values
        for &ret in returns {
            if ret.is_nan() || risk_free_rate.is_nan() {
                return Err(StatsError::InvalidInput(
                    "NaN detected in input values".to_string(),
                ));
            }
        }
    }

    // Compute the sum of the returns.
    let sum = returns.iter().fold(T::zero(), |acc, &x| acc + x);

    // Convert the number of returns to the generic type T.
    let count = T::from_usize(returns.len()).ok_or_else(|| {
        StatsError::ConversionError("Conversion from usize to generic type T failed".to_string())
    })?;

    // Calculate the mean return.
    let mean = sum / count;

    // Compute the sum of squared differences from the mean.
    let sum_sq_diff = returns.iter().fold(T::zero(), |acc, &x| {
        let diff = x - mean;
        acc + diff * diff
    });

    // Calculate sample variance using (n - 1) as denominator.
    let n_minus_one = returns.len() - 1;
    let divisor = T::from_usize(n_minus_one).ok_or_else(|| {
        StatsError::ConversionError("Conversion from usize to generic type T failed".to_string())
    })?;
    let variance = sum_sq_diff / divisor;
    let std_dev = variance.sqrt();

    // Ensure the standard deviation is not (effectively) zero to avoid division by zero.
    if std_dev.abs() <= T::epsilon() {
        return Err(StatsError::InvalidInput(
            "Standard deviation is effectively zero due to floating point precision, Sharpe ratio \
             is undefined"
                .to_string(),
        ));
    }

    // Compute the Sharpe ratio: (mean return - risk free rate) / standard deviation.
    let sharpe_ratio = (mean - risk_free_rate) / std_dev;
    Ok(sharpe_ratio)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_sharpe_ratio_valid() {
        // Given a set of returns and a risk-free rate:
        // returns = [0.1, 0.2, 0.15, 0.05, 0.1]
        // mean   = (0.1 + 0.2 + 0.15 + 0.05 + 0.1) / 5 = 0.12
        // Squared differences: [0.0004, 0.0064, 0.0009, 0.0049, 0.0004]
        // Sum = 0.013, sample variance = 0.013 / 4 = 0.00325, standard deviation ≈ 0.05700877
        // Sharpe ratio = (0.12 - 0.05) / 0.05700877 ≈ 1.22807
        let returns = vec![0.1, 0.2, 0.15, 0.05, 0.1];
        let risk_free_rate = 0.05_f64;
        let result = sharpe(&returns, risk_free_rate).unwrap();
        assert_relative_eq!(result, 1.227881227, max_relative = 1e-5);
    }

    #[test]
    fn test_sharpe_ratio_insufficient_data() {
        // Test error when less than two data points are provided.
        let returns = vec![0.1];
        let risk_free_rate = 0.05;
        let result = sharpe(&returns, risk_free_rate);
        assert!(result.is_err());
    }

    #[test]
    fn test_sharpe_ratio_zero_std_dev() {
        // Test error when all returns are equal, resulting in near-zero standard deviation.
        let returns = vec![0.1, 0.1, 0.1];
        let risk_free_rate = 0.05;
        let result = sharpe(&returns, risk_free_rate);
        assert!(result.is_err());
    }
}
