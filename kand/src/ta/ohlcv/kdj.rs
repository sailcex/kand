use crate::{helper::{highest_bars, lowest_bars}, KandError, TAFloat};

/// Returns the lookback period required for KDJ calculation.
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(param_period - 1)
}

/// # Description
/// KDJ 指标（随机指标）的初始值需要通过一个固定长度的窗口来计算 RSV（Raw Stochastic Value），
/// 因此回溯期等于 `period - 1`，也就是窗口的最后一个索引位置。
///
/// # Mathematical Formula
/// ```text
/// RSVₜ = (Closeₜ - LowestLowₙ) / (HighestHighₙ - LowestLowₙ) * 100
/// Kₜ   = α · RSVₜ + (1 - α) · Kₜ₋₁
/// Dₜ   = α · Kₜ   + (1 - α) · Dₜ₋₁
/// Jₜ   = 3·Kₜ - 2·Dₜ
/// where:
///   n     = period (RSV 窗口长度)
///   α     = 1/3 (平滑常数)
///   LowestLowₙ  = 最近 n 个周期的最低价
///   HighestHighₙ = 最近 n 个周期的最高价
/// ```
///
/// # Calculation Principle
/// 1. **RSV 计算**：对每个索引 i ≥ lookback，取 `i−(period−1)` 到 `i` 范围内的最高价和最低价，
///    计算当期 RSV。  
/// 2. **初始 K、D**：在 `i = lookback` 处，令  
///    ```text
///    K₀ = RSV₀,  D₀ = RSV₀  
///    J₀ = 3·K₀ - 2·D₀ = RSV₀  
///    ```  
/// 3. **后续平滑**：对 i > lookback，按 Wilder 平滑：  
///    ```text
///    Kᵢ = (1−α)·Kᵢ₋₁ + α·RSVᵢ  
///    Dᵢ = (1−α)·Dᵢ₋₁ + α·Kᵢ  
///    Jᵢ = 3·Kᵢ - 2·Dᵢ  
///    ```  
/// 4. **填充 NaN**：在索引 < lookback 处，K、D、J 均设为 NaN。
///
/// # Arguments
/// * `input_high`      – 最高价数组  
/// * `input_low`       – 最低价数组  
/// * `input_close`     – 收盘价数组  
/// * `param_period` – RSV 窗口长度，必须 ≥ 2  
/// * `output_k`   – 存储 K 值的数组，长度等于输入长度  
/// * `output_d`   – 存储 D 值的数组，长度等于输入长度  
/// * `output_j`   – 存储 J 值的数组，长度等于输入长度
///
/// # Returns
/// * `Result<(), KandError>` – 成功时返回 Ok，失败返回对应错误。
///
/// # Errors
/// * `KandError::InvalidParameter` – 当 `param_period < 2`  
/// * `KandError::InvalidData`      – 当任一输入数组为空  
/// * `KandError::LengthMismatch`   – 当任一输出数组与输入长度不符  
/// * `KandError::InsufficientData` – 当输入长度 ≤ lookback  
/// * `KandError::NaNDetected`      – （`deep-check` 模式）若存在 NaN
///
/// # Example
/// ```rust
/// use kand::ohlcv::kdj;
///
/// let input_high  = vec![10.0, 10.5, 10.2, 10.8, 11.0, 10.7, 11.2, 11.5, 11.3];
/// let input_low   = vec![ 9.5,  9.8,  9.7, 10.1, 10.5, 10.2, 10.8, 10.9, 11.1];
/// let input_close = vec![10.0, 10.3, 10.0, 10.7, 10.8, 10.5, 11.0, 11.4, 11.2];
/// let param_period = 9;
///
/// let mut output_k = vec![0.0; input_high.len()];
/// let mut output_d = vec![0.0; input_high.len()];
/// let mut output_j = vec![0.0; input_high.len()];
///
/// kdj::kdj(
///     &input_high,
///     &input_low,
///     &input_close,
///     param_period,
///     &mut output_k,
///     &mut output_d,
///     &mut output_j,
/// ).unwrap();
/// ```
pub fn kdj(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    param_period: usize,
    output_k: &mut [TAFloat],
    output_d: &mut [TAFloat],
    output_j: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_close.len();
    let lookback = lookback(param_period)?;

    #[cfg(feature = "check")]
    {
        if len == 0 {
            return Err(KandError::InvalidData);
        }
        if input_high.len() != len || input_low.len() != len {
            return Err(KandError::LengthMismatch);
        }
        if output_k.len() != len || output_d.len() != len || output_j.len() != len {
            return Err(KandError::LengthMismatch);
        }
        if len <= lookback {
            return Err(KandError::InsufficientData);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        for &v in input_high.iter()
            .chain(input_low.iter())
            .chain(input_close.iter())
        {
            if v.is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    const ALPHA: TAFloat = 1.0 / 3.0;

    // 1. 从 lookback 开始计算
    for i in lookback..len {
        // 找窗口内最高/最低价
        let highest_idx = highest_bars(input_high, i, param_period)?;
        let lowest_idx = lowest_bars(input_low, i, param_period)?;

        let highest = input_high[i - highest_idx];
        let lowest  = input_low[i - lowest_idx];

        // 计算 RSV
        let rsv = (input_close[i] - lowest) / (highest - lowest) * 100.0;

        if i == lookback {
            // 初始 K, D, J
            output_k[i] = rsv;
            output_d[i] = rsv;
            output_j[i] = 3.0 * rsv - 2.0 * rsv;
        } else {
            // Wilder 平滑
            let prev_k = output_k[i - 1];
            let prev_d = output_d[i - 1];
            let k_val = prev_k * (1.0 - ALPHA) + rsv * ALPHA;
            let d_val = prev_d * (1.0 - ALPHA) + k_val * ALPHA;
            output_k[i] = k_val;
            output_d[i] = d_val;
            output_j[i] = 3.0 * k_val - 2.0 * d_val;
        }
    }

    // 2. 填充 NaN
    for i in 0..lookback {
        output_k[i] = TAFloat::NAN;
        output_d[i] = TAFloat::NAN;
        output_j[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Incremental KDJ calculation using previous K & D and current RSV.
///
/// # Mathematical Formula
/// ```text
/// K = (1−α)·Kₚᵣₑᵥ + α·RSV
/// D = (1−α)·Dₚᵣₑᵥ + α·K
/// J = 3·K − 2·D
/// where α = 1/3
/// ```
///
/// # Arguments
/// * `rsv`    – 当前周期的 RSV 值  
/// * `prev_k` – 上一周期的 K 值  
/// * `prev_d` – 上一周期的 D 值  
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat), KandError>`  
///   - `(k, d, j)`
///
/// # Errors
/// * `KandError::NaNDetected` – （`deep-check`）任一输入为 NaN
pub fn kdj_inc(
    rsv: TAFloat,
    prev_k: TAFloat,
    prev_d: TAFloat,
) -> Result<(TAFloat, TAFloat, TAFloat), KandError> {
    #[cfg(feature = "deep-check")]
    {
        if rsv.is_nan() || prev_k.is_nan() || prev_d.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }
    const ALPHA: TAFloat = 1.0 / 3.0;
    let k = prev_k * (1.0 - ALPHA) + rsv * ALPHA;
    let d = prev_d * (1.0 - ALPHA) + k * ALPHA;
    let j = 3.0 * k - 2.0 * d;
    Ok((k, d, j))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;

    #[test]
    fn test_kdj_incremental() {
        let rsvs = vec![90.0, 85.0, 80.0];
        let mut pk = rsvs[0];
        let mut pd = rsvs[0];
        for &r in &rsvs[1..] {
            let (k, d, j) = kdj_inc(r, pk, pd).unwrap();
            // 手动计算
            let expect_k = pk * 2.0/3.0 + r * 1.0/3.0;
            let expect_d = pd * 2.0/3.0 + expect_k * 1.0/3.0;
            let expect_j = 3.0 * expect_k - 2.0 * expect_d;
            assert_relative_eq!(k, expect_k, epsilon = 1e-6);
            assert_relative_eq!(d, expect_d, epsilon = 1e-6);
            assert_relative_eq!(j, expect_j, epsilon = 1e-6);
            pk = k; pd = d;
        }
    }
}
