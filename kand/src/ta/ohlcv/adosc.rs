use num_traits::{Float, FromPrimitive};

use super::{ad, ema};
use crate::KandError;

/// Get the lookback period for A/D Oscillator calculation
///
/// Returns the number of data points needed before first valid output.
///
/// # Arguments
/// * `param_fast_period` - Fast period for EMA calculation
/// * `param_slow_period` - Slow period for EMA calculation
///
/// # Returns
/// * `Result<usize, KandError>` - Number of data points needed before first valid output
///
/// # Errors
/// * `KandError::InvalidParameter` - If `fast_period` or `slow_period` is 0, or if `fast_period` >= `slow_period`
///
/// # Example
/// ```
/// use kand::ohlcv::adosc::lookback;
/// let lookback_period = lookback(3, 10).unwrap();
/// assert_eq!(lookback_period, 9);
/// ```
pub const fn lookback(
    param_fast_period: usize,
    param_slow_period: usize,
) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if param_fast_period < 2 || param_slow_period < 2 {
            return Err(KandError::InvalidParameter);
        }
        if param_fast_period >= param_slow_period {
            return Err(KandError::InvalidParameter);
        }
    }

    ema::lookback(param_slow_period)
}

/// Calculate Accumulation/Distribution Oscillator (A/D Oscillator or ADOSC)
///
/// The A/D Oscillator is a momentum indicator that measures the difference between fast and slow EMAs of the
/// Accumulation/Distribution Line. It helps identify trend strength and potential reversals.
///
/// # Mathematical Formula
/// ```text
/// Money Flow Multiplier = ((Close - Low) - (High - Close)) / (High - Low)
/// Money Flow Volume = Money Flow Multiplier * Volume
/// AD = Cumulative sum of Money Flow Volume
/// ADOSC = EMA(AD, fast_period) - EMA(AD, slow_period)
/// ```
///
/// # Calculation Steps
/// 1. Calculate Money Flow Multiplier for each period
/// 2. Multiply by volume to get Money Flow Volume
/// 3. Calculate cumulative AD line
/// 4. Calculate fast and slow EMAs of AD line
/// 5. Subtract slow EMA from fast EMA to get ADOSC
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `input_close` - Array of closing prices
/// * `input_volume` - Array of volume data
/// * `param_fast_period` - Fast EMA period
/// * `param_slow_period` - Slow EMA period
/// * `output_adosc` - Output array for ADOSC values
/// * `output_ad` - Output array for AD line values
/// * `output_ad_fast_ema` - Output array for fast EMA values of AD line
/// * `output_ad_slow_ema` - Output array for slow EMA values of AD line
///
/// # Returns
/// * `Result<(), KandError>` - Ok if calculation succeeds
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input/output arrays have different lengths
/// * `KandError::InvalidParameter` - If `fast_period` or `slow_period` is 0, or if `fast_period` >= `slow_period`
/// * `KandError::InsufficientData` - If input length < `slow_period`
/// * `KandError::NaNDetected` - If any input contains NaN values (when `deep-check` enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::adosc::adosc;
///
/// let high = vec![10.0, 11.0, 12.0, 11.5, 10.5];
/// let low = vec![8.0, 9.0, 10.0, 9.5, 8.5];
/// let close = vec![9.0, 10.0, 11.0, 10.0, 9.0];
/// let volume = vec![100.0, 150.0, 200.0, 150.0, 100.0];
/// let mut adosc_out = vec![0.0; 5];
/// let mut ad_out = vec![0.0; 5];
/// let mut ad_fast_ema = vec![0.0; 5];
/// let mut ad_slow_ema = vec![0.0; 5];
///
/// adosc(
///     &high,
///     &low,
///     &close,
///     &volume,
///     3,
///     5,
///     &mut adosc_out,
///     &mut ad_out,
///     &mut ad_fast_ema,
///     &mut ad_slow_ema,
/// )
/// .unwrap();
/// ```
pub fn adosc<T>(
    input_high: &[T],
    input_low: &[T],
    input_close: &[T],
    input_volume: &[T],
    param_fast_period: usize,
    param_slow_period: usize,
    output_adosc: &mut [T],
    output_ad: &mut [T],
    output_ad_fast_ema: &mut [T],
    output_ad_slow_ema: &mut [T],
) -> Result<(), KandError>
where
    T: Float + FromPrimitive,
{
    let len = input_high.len();
    let lookback = lookback(param_fast_period, param_slow_period)?;

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
        if len != input_low.len()
            || len != input_close.len()
            || len != input_volume.len()
            || len != output_adosc.len()
            || len != output_ad.len()
            || len != output_ad_fast_ema.len()
            || len != output_ad_slow_ema.len()
        {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        for i in 0..len {
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

    // Calculate A/D line first
    ad::ad(input_high, input_low, input_close, input_volume, output_ad)?;

    // Calculate fast and slow EMAs of A/D line
    ema::ema(output_ad, param_fast_period, None, output_ad_fast_ema)?;
    ema::ema(output_ad, param_slow_period, None, output_ad_slow_ema)?;

    // Calculate ADOSC = Fast EMA - Slow EMA
    for i in lookback..len {
        output_adosc[i] = output_ad_fast_ema[i] - output_ad_slow_ema[i];
    }

    // Fill initial values with NAN
    for x in output_adosc.iter_mut().take(lookback) {
        *x = T::nan();
    }

    Ok(())
}

/// Calculate latest A/D Oscillator value incrementally
///
/// Provides optimized calculation of the latest ADOSC value when new data arrives,
/// without recalculating the entire series.
///
/// # Mathematical Formula
/// ```text
/// Money Flow Multiplier = ((Close - Low) - (High - Close)) / (High - Low)
/// Money Flow Volume = Money Flow Multiplier * Volume
/// AD = Previous AD + Money Flow Volume
/// Fast_EMA = (AD - prev_fast_ema) * (2/(fast_period+1)) + prev_fast_ema
/// Slow_EMA = (AD - prev_slow_ema) * (2/(slow_period+1)) + prev_slow_ema
/// ADOSC = Fast_EMA - Slow_EMA
/// ```
///
/// # Arguments
/// * `input_high` - Current high price
/// * `input_low` - Current low price
/// * `input_close` - Current close price
/// * `input_volume` - Current volume
/// * `input_prev_ad` - Previous AD value
/// * `input_prev_ad_fast_ema` - Previous fast EMA value of AD line
/// * `input_prev_ad_slow_ema` - Previous slow EMA value of AD line
/// * `param_fast_period` - Fast EMA period
/// * `param_slow_period` - Slow EMA period
///
/// # Returns
/// * `Result<(T, T, T, T), KandError>` - Tuple of (ADOSC, AD, Fast EMA, Slow EMA)
///
/// # Errors
/// * `KandError::InvalidParameter` - If `fast_period` or `slow_period` is 0, or if `fast_period` >= `slow_period`
/// * `KandError::NaNDetected` - If any input contains NaN values (when `deep-check` enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::adosc::adosc_incremental;
///
/// let (adosc, ad, ad_fast_ema, ad_slow_ema) = adosc_incremental(
///     10.5,  // high
///     9.5,   // low
///     10.0,  // close
///     150.0, // volume
///     100.0, // prev_ad
///     95.0,  // prev_ad_fast_ema
///     90.0,  // prev_ad_slow_ema
///     3,     // fast_period
///     10,    // slow_period
/// )
/// .unwrap();
/// ```
pub fn adosc_incremental<T>(
    input_high: T,
    input_low: T,
    input_close: T,
    input_volume: T,
    input_prev_ad: T,
    input_prev_ad_fast_ema: T,
    input_prev_ad_slow_ema: T,
    param_fast_period: usize,
    param_slow_period: usize,
) -> Result<(T, T, T, T), KandError>
where
    T: Float + FromPrimitive,
{
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if param_fast_period == 0 || param_slow_period == 0 {
            return Err(KandError::InvalidParameter);
        }
        if param_fast_period >= param_slow_period {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        // NaN check
        if input_high.is_nan()
            || input_low.is_nan()
            || input_close.is_nan()
            || input_volume.is_nan()
            || input_prev_ad.is_nan()
            || input_prev_ad_fast_ema.is_nan()
            || input_prev_ad_slow_ema.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let output_ad = ad::ad_incremental(
        input_high,
        input_low,
        input_close,
        input_volume,
        input_prev_ad,
    )?;
    let output_ad_fast_ema =
        ema::ema_incremental(output_ad, input_prev_ad_fast_ema, param_fast_period, None)?;
    let output_ad_slow_ema =
        ema::ema_incremental(output_ad, input_prev_ad_slow_ema, param_slow_period, None)?;
    let output_adosc = output_ad_fast_ema - output_ad_slow_ema;

    Ok((
        output_adosc,
        output_ad,
        output_ad_fast_ema,
        output_ad_slow_ema,
    ))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Basic functionality tests
    #[test]
    fn test_adosc_calculation() {
        let input_high = vec![
            35266.0, 35247.5, 35235.7, 35190.8, 35182.0, 35258.0, 35262.9, 35281.5, 35256.0,
            35210.0, 35185.4, 35230.0, 35241.0, 35218.1, 35212.6, 35128.9, 35047.7, 35019.5,
            35078.8, 35085.0, 35034.1, 34984.4, 35010.8, 35047.1, 35091.4, 35150.4, 35123.9,
            35110.0, 35092.1, 35179.2, 35244.9, 35150.2, 35136.0, 35133.6, 35188.0, 35215.3,
            35221.9, 35219.2, 35234.0, 35216.7, 35197.9, 35178.4, 35183.4, 35129.7, 35149.1,
            35129.3, 35125.5, 35114.5, 35120.1, 35129.4, 35105.4, 35054.1, 35034.6, 35032.9,
            35070.8, 35086.0, 35086.9, 35048.9, 34988.6, 35004.3, 34985.0, 35004.2, 35010.0,
            35041.8, 35024.7, 34982.0, 35018.0, 34978.2, 34959.5, 34965.0, 34985.3, 35002.4,
            35018.0, 34989.0, 34943.0, 34900.0, 34932.1, 34930.0, 34920.3, 34929.9, 34940.0,
            35019.7, 35009.1, 34980.2, 34977.3, 34976.1, 34969.4, 35000.0, 35010.0, 35015.9,
            35062.9, 35084.8, 35085.1, 35077.9, 35118.0, 35104.0, 35086.2, 35041.7, 35009.2,
            34994.2,
        ];
        let input_low = vec![
            35216.1, 35206.5, 35180.0, 35130.7, 35153.6, 35174.7, 35202.6, 35203.5, 35175.0,
            35166.0, 35170.9, 35154.1, 35186.0, 35143.9, 35080.1, 35021.1, 34950.1, 34966.0,
            35012.3, 35022.2, 34931.6, 34911.0, 34952.5, 34977.9, 35039.0, 35073.0, 35055.0,
            35084.0, 35060.0, 35073.1, 35090.0, 35072.0, 35078.0, 35088.0, 35124.8, 35169.4,
            35138.0, 35141.0, 35182.0, 35151.1, 35158.4, 35140.0, 35087.0, 35085.8, 35114.7,
            35086.0, 35090.6, 35074.1, 35078.4, 35100.0, 35030.2, 34986.3, 34988.1, 34973.1,
            35012.3, 35048.3, 35038.9, 34937.3, 34937.0, 34958.7, 34925.0, 34910.0, 34981.6,
            34980.2, 34982.0, 34940.9, 34970.0, 34924.7, 34922.1, 34914.0, 34955.8, 34975.0,
            34975.0, 34926.0, 34865.1, 34821.0, 34830.4, 34883.5, 34888.5, 34904.6, 34880.6,
            34934.0, 34978.5, 34965.9, 34936.4, 34942.5, 34945.0, 34969.3, 34983.8, 35003.9,
            35001.1, 35032.1, 35027.3, 35062.3, 35067.8, 35070.7, 35030.2, 34981.0, 34970.5,
            34974.5,
        ];
        let input_close = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0, 35114.5, 35097.2,
            35092.0, 35073.2, 35139.3, 35092.0, 35126.7, 35106.3, 35124.8, 35170.1, 35215.3,
            35154.0, 35216.3, 35211.8, 35158.4, 35172.0, 35176.7, 35113.3, 35114.7, 35129.3,
            35094.6, 35114.4, 35094.5, 35116.0, 35105.4, 35050.7, 35031.3, 35008.1, 35021.4,
            35048.4, 35080.1, 35043.6, 34962.7, 34970.1, 34980.1, 34930.6, 35000.0, 34998.0,
            35024.7, 34982.1, 34972.3, 34971.6, 34953.0, 34937.0, 34964.3, 34975.1, 34995.1,
            34989.0, 34942.9, 34895.2, 34830.4, 34925.1, 34888.6, 34910.3, 34917.6, 34940.0,
            35005.4, 34980.1, 34966.8, 34976.1, 34948.6, 34969.3, 34996.5, 35004.0, 35011.0,
            35059.2, 35036.1, 35062.3, 35067.7, 35087.9, 35076.7, 35041.6, 34993.3, 34974.5,
            34990.2,
        ];
        let input_volume = vec![
            1055.365, 756.488, 682.152, 1197.747, 425.97, 859.638, 741.925, 888.477, 1043.333,
            467.901, 387.47, 566.099, 672.296, 834.915, 1854.024, 3670.795, 3761.198, 1605.442,
            1726.574, 934.713, 2199.061, 2349.823, 837.218, 1000.638, 1218.202, 2573.668, 1098.409,
            609.582, 670.489, 1637.998, 2682.922, 923.588, 554.766, 510.261, 882.672, 1087.53,
            1164.362, 991.265, 1042.659, 748.721, 469.772, 419.244, 896.583, 736.185, 510.968,
            503.042, 376.2, 592.877, 580.915, 333.615, 1106.869, 1761.343, 506.403, 1181.917,
            817.219, 727.725, 723.652, 1702.198, 769.212, 414.213, 702.499, 1083.179, 411.098,
            971.148, 774.147, 376.625, 333.361, 666.541, 418.598, 836.645, 506.807, 418.69,
            606.013, 658.819, 1776.331, 1757.305, 985.24, 607.588, 350.444, 402.724, 476.235,
            1899.96, 546.185, 233.707, 612.487, 313.292, 167.004, 298.175, 397.43, 194.525,
            685.384, 737.572, 576.129, 264.406, 577.913, 314.803, 694.229, 1253.468, 466.235,
            248.839,
        ];
        let param_fast_period = 3;
        let param_slow_period = 10;
        let mut output_adosc = vec![0.0; input_high.len()];
        let mut output_ad = vec![0.0; input_high.len()];
        let mut output_ad_fast_ema = vec![0.0; input_high.len()];
        let mut output_ad_slow_ema = vec![0.0; input_high.len()];

        adosc(
            &input_high,
            &input_low,
            &input_close,
            &input_volume,
            param_fast_period,
            param_slow_period,
            &mut output_adosc,
            &mut output_ad,
            &mut output_ad_fast_ema,
            &mut output_ad_slow_ema,
        )
        .unwrap();

        // First 9 values should be NaN (slow_period - 1)
        for value in output_adosc.iter().take(9) {
            assert!(value.is_nan());
        }

        // Check some known values
        let expected_values = [
            230.315_727_147_474_04,
            137.903_251_057_287_2,
            23.492_245_181_438_648,
            -156.404_749_966_073_5,
            -452.976_230_723_257_23,
            -650.802_620_187_369_5,
            -625.544_386_492_415,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_adosc[93 + i], *expected, epsilon = 0.00001);
        }

        // Now test incremental calculation matches regular calculation starting from index 9
        let mut prev_ad = output_ad[9];
        let mut prev_ad_fast_ema = output_ad_fast_ema[9];
        let mut prev_ad_slow_ema = output_ad_slow_ema[9];

        // Test each incremental step starting from index 10
        for i in 10..14 {
            let (output_adosc_inc, output_ad_inc, output_ad_fast_ema_inc, output_ad_slow_ema_inc) =
                adosc_incremental(
                    input_high[i],
                    input_low[i],
                    input_close[i],
                    input_volume[i],
                    prev_ad,
                    prev_ad_fast_ema,
                    prev_ad_slow_ema,
                    param_fast_period,
                    param_slow_period,
                )
                .unwrap();
            assert_relative_eq!(output_adosc_inc, output_adosc[i], epsilon = 0.00001);
            assert_relative_eq!(output_ad_inc, output_ad[i], epsilon = 0.00001);
            assert_relative_eq!(
                output_ad_fast_ema_inc,
                output_ad_fast_ema[i],
                epsilon = 0.00001
            );
            assert_relative_eq!(
                output_ad_slow_ema_inc,
                output_ad_slow_ema[i],
                epsilon = 0.00001
            );
            prev_ad = output_ad_inc;
            prev_ad_fast_ema = output_ad_fast_ema_inc;
            prev_ad_slow_ema = output_ad_slow_ema_inc;
        }
    }
}
