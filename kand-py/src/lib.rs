use pyo3::prelude::*;

pub mod ta;

/// A Python module implemented in Rust.
#[rustfmt::skip]
#[pymodule]
fn kand(m: &Bound<'_, PyModule>) -> PyResult<()> {

    // Add all OHLCV functions
    m.add_function(wrap_pyfunction!(ta::ohlcv::ad::ad_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::ad::ad_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::adosc::adosc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::adosc::adosc_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::adx::adx_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::adx::adx_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::adxr::adxr_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::adxr::adxr_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::aroon::aroon_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::aroon::aroon_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::aroonosc::aroonosc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::aroonosc::aroonosc_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::atr::atr_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::atr::atr_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::bbands::bbands_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::bbands::bbands_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::bop::bop_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::bop::bop_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cci::cci_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cci::cci_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_doji::cdl_doji_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_doji::cdl_doji_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_dragonfly_doji::cdl_dragonfly_doji_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_dragonfly_doji::cdl_dragonfly_doji_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_gravestone_doji::cdl_gravestone_doji_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_gravestone_doji::cdl_gravestone_doji_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_hammer::cdl_hammer_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_hammer::cdl_hammer_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_inverted_hammer::cdl_inverted_hammer_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_inverted_hammer::cdl_inverted_hammer_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_long_shadow::cdl_long_shadow_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_long_shadow::cdl_long_shadow_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_marubozu::cdl_marubozu_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_marubozu::cdl_marubozu_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::dema::dema_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::dema::dema_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::dx::dx_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::dx::dx_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::ecl::ecl_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::ecl::ecl_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::ema::ema_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::ema::ema_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::macd::macd_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::macd::macd_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::medprice::medprice_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::medprice::medprice_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::mfi::mfi_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::midpoint::midpoint_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::midpoint::midpoint_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::midprice::midprice_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::midprice::midprice_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::minus_di::minus_di_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::minus_dm::minus_dm_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::mom::mom_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::mom::mom_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::natr::natr_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::natr::natr_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::obv::obv_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::obv::obv_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::plus_di::plus_di_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::plus_di::plus_di_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::plus_dm::plus_dm_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::plus_dm::plus_dm_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rma::rma_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rma::rma_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::roc::roc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::roc::roc_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rocp::rocp_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rocp::rocp_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rocr::rocr_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rocr::rocr_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rocr100::rocr100_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rocr100::rocr100_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rsi::rsi_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rsi::rsi_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::sar::sar_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::sar::sar_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::sma::sma_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::sma::sma_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::stoch::stoch_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::supertrend::supertrend_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::supertrend::supertrend_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::t3::t3_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::t3::t3_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::tema::tema_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::tema::tema_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::trange::trange_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::trange::trange_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::trima::trima_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::trima::trima_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::trix::trix_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::trix::trix_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::typprice::typprice_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::typprice::typprice_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::vegas::vegas_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::vegas::vegas_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::wclprice::wclprice_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::wclprice::wclprice_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::willr::willr_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::willr::willr_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::wma::wma_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::wma::wma_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::vwap::vwap_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::vwap::vwap_incremental_py, m)?)?;

    // Add all stats functions
    m.add_function(wrap_pyfunction!(ta::stats::max::max_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::max::max_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::min::min_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::min::min_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::stddev::stddev_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::stddev::stddev_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::sum::sum_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::sum::sum_incremental_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::var::var_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::var::var_incremental_py, m)?)?;

    // Add all helper functions

    Ok(())
}
