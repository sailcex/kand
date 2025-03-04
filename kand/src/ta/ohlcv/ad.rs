use crate::{KandError, TAFloat};

/// Returns the lookback period required for A/D calculation
///
/// # Description
/// The A/D indicator requires no lookback period as it can be calculated from the first data point.
///
/// # Arguments
/// * None
///
/// # Returns
/// * `Result<usize, KandError>` - Returns 0 as no lookback period is needed
///
/// # Errors
/// * None
///
/// # Example
/// ```
/// use kand::ohlcv::ad;
/// let lookback = ad::lookback().unwrap();
/// assert_eq!(lookback, 0);
/// ```
pub const fn lookback() -> Result<usize, KandError> {
    Ok(0)
}

/// Calculates the Accumulation/Distribution (A/D) indicator for the entire price series
///
/// # Description
/// The A/D indicator measures the cumulative flow of money into and out of a security by analyzing
/// price and volume data.
///
/// # Mathematical Formula
/// ```text
/// Money Flow Multiplier (MFM) = ((Close - Low) - (High - Close)) / (High - Low)
/// Money Flow Volume (MFV) = MFM * Volume
/// A/D = Previous A/D + MFV
/// ```
///
/// # Calculation Principle
/// 1. Calculate Money Flow Multiplier (MFM):
///    - Ranges from -1 to +1
///    - Close near high indicates buying pressure (+1)
///    - Close near low indicates selling pressure (-1)
/// 2. Weight volume by MFM to get Money Flow Volume (MFV)
/// 3. Sum MFV values to create cumulative A/D line
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `input_close` - Array of closing prices
/// * `input_volume` - Array of volume data
/// * `output_ad` - Array to store calculated A/D values
///
/// # Returns
/// * `Result<(), KandError>` - Ok if calculation succeeds
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input arrays have different lengths
/// * `KandError::NaNDetected` - If any input contains NaN values
///
/// # Example
/// ```
/// use kand::ohlcv::ad;
/// let input_high = vec![10.0, 12.0, 15.0];
/// let input_low = vec![8.0, 9.0, 11.0];
/// let input_close = vec![9.0, 11.0, 13.0];
/// let input_volume = vec![100.0, 150.0, 200.0];
/// let mut output_ad = vec![0.0; 3];
///
/// ad::ad(
///     &input_high,
///     &input_low,
///     &input_close,
///     &input_volume,
///     &mut output_ad,
/// )
/// .unwrap();
/// ```
#[allow(clippy::similar_names)]
pub fn ad(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    input_volume: &[TAFloat],
    output_ad: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_high.len();
    let lookback = lookback()?;

    #[cfg(feature = "check")]
    {
        // Check for empty input
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Check length consistency
        if len != input_low.len()
            || len != input_close.len()
            || len != input_volume.len()
            || len != output_ad.len()
        {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        for i in lookback..len {
            // NaN check
            if input_high[i].is_nan()
                || input_low[i].is_nan()
                || input_close[i].is_nan()
                || input_volume[i].is_nan()
            {
                return Err(KandError::NaNDetected);
            }
        }
    }

    let mut ad = 0.0;
    for i in lookback..len {
        let high_low_diff = input_high[i] - input_low[i];
        let mfm = if high_low_diff == 0.0 {
            0.0
        } else {
            ((input_close[i] - input_low[i]) - (input_high[i] - input_close[i])) / high_low_diff
        };
        let mfv = mfm * input_volume[i];
        ad += mfv;
        output_ad[i] = ad;
    }

    Ok(())
}

/// Calculates the latest A/D value incrementally using the previous A/D value
///
/// # Description
/// Optimized version that calculates only the latest A/D value using the previous A/D value,
/// avoiding recalculation of the entire series.
///
/// # Mathematical Formula
/// ```text
/// Money Flow Multiplier (MFM) = ((Close - Low) - (High - Close)) / (High - Low)
/// Money Flow Volume (MFV) = MFM * Volume
/// Latest A/D = Previous A/D + MFV
/// ```
///
/// # Arguments
/// * `input_high` - Latest high price
/// * `input_low` - Latest low price
/// * `input_close` - Latest closing price
/// * `input_volume` - Latest volume
/// * `prev_ad` - Previous A/D value
///
/// # Returns
/// * `Result<TAFloat, KandError>` - Latest A/D value if calculation succeeds
///
/// # Errors
/// * `KandError::NaNDetected` - If any input contains NaN values
///
/// # Example
/// ```
/// use kand::ohlcv::ad;
/// let input_high = 15.0;
/// let input_low = 11.0;
/// let input_close = 13.0;
/// let input_volume = 200.0;
/// let prev_ad = 25.0;
///
/// let output_ad = ad::ad_inc(input_high, input_low, input_close, input_volume, prev_ad).unwrap();
/// ```
pub fn ad_inc(
    input_high: TAFloat,
    input_low: TAFloat,
    input_close: TAFloat,
    input_volume: TAFloat,
    prev_ad: TAFloat,
) -> Result<TAFloat, KandError> {
    #[cfg(feature = "deep-check")]
    {
        // NaN check
        if input_high.is_nan()
            || input_low.is_nan()
            || input_close.is_nan()
            || input_volume.is_nan()
            || prev_ad.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let high_low_diff = input_high - input_low;
    let mfm = if high_low_diff == 0.0 {
        0.0
    } else {
        ((input_close - input_low) - (input_high - input_close)) / high_low_diff
    };
    let mfv = mfm * input_volume;
    Ok(prev_ad + mfv)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    /// Test the calculation of A/D
    #[test]
    fn test_ad_calculation() {
        let input_high = vec![
            35266.0, 35247.5, 35235.7, 35190.8, 35182.0, 35258.0, 35262.9, 35281.5, 35256.0,
            35210.0, 35185.4, 35230.0, 35241.0, 35218.1, 35212.6, 35128.9, 35047.7, 35019.5,
            35078.8, 35085.0, 35034.1, 34984.4, 35010.8, 35047.1, 35091.4,
        ];
        let input_low = vec![
            35216.1, 35206.5, 35180.0, 35130.7, 35153.6, 35174.7, 35202.6, 35203.5, 35175.0,
            35166.0, 35170.9, 35154.1, 35186.0, 35143.9, 35080.1, 35021.1, 34950.1, 34966.0,
            35012.3, 35022.2, 34931.6, 34911.0, 34952.5, 34977.9, 35039.0,
        ];
        let input_close = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0,
        ];
        let input_volume = vec![
            1055.365, 756.488, 682.152, 1197.747, 425.97, 859.638, 741.925, 888.477, 1043.333,
            467.901, 387.47, 566.099, 672.296, 834.915, 1854.024, 3670.795, 3761.198, 1605.442,
            1726.574, 934.713, 2199.061, 2349.823, 837.218, 1000.638, 1218.202,
        ];
        let mut output_ad = vec![0.0; input_high.len()];

        ad(
            &input_high,
            &input_low,
            &input_close,
            &input_volume,
            &mut output_ad,
        )
        .unwrap();

        // Test remaining values in a loop
        let expected_values = [
            -1055.365,
            -1_262.015_380_487_751_1,
            -1_682.083_847_274_164_3,
            -1_313.393_007_007_977,
            -902.421_950_669_948,
            -112.958_481_282_220_53,
            -849.961_922_409_808_2,
            -635.816_183_948_236_6,
            -1_096.943_608_639_632_2,
            -1_167.128_758_639_694,
            -1_330.133_379_329_504_8,
            -765.526_076_299_18,
            -789.973_203_571_907_2,
            -1_246.813_486_590_858_5,
            -2_815.387_753_760_547_6,
            -5_117.296_306_636_379,
            -5_086.466_814_832_707,
            -3_847.125_607_355_984_4,
            -2_629.436_575_777_188,
            -3_492.706_543_930_014,
            -5_352.790_336_125_074,
            -5_039.053_750_294_157,
            -4_512.022_865_217_038,
            -3_664.661_784_292_062_7,
            -2_976.517_143_070_741_4,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_ad[i], *expected, epsilon = 0.00001);
        }

        // Now test incremental calculation matches regular calculation
        let mut prev_ad = output_ad[0];

        // Test each incremental step
        for i in 1..input_high.len() {
            let result = ad_inc(
                input_high[i],
                input_low[i],
                input_close[i],
                input_volume[i],
                prev_ad,
            )
            .unwrap();
            assert_relative_eq!(result, output_ad[i], epsilon = 0.00001);
            prev_ad = result;
        }
    }
}
