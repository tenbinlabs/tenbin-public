use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{TryFromInto, serde_as};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paginated<T> {
    pub results: Vec<T>,
    #[serde(default)]
    pub next_page: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtcBalanceEntry {
    pub asset: String,
    #[serde(with = "rust_decimal::serde::float")]
    pub quantity: Decimal,
    pub settlement_datetime: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtcBalanceAccount {
    pub id: String,
    #[serde(rename = "type")]
    pub account_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtcBalancesRecord {
    pub account: OtcBalanceAccount,
    pub balances: Vec<OtcBalanceEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtcBalancesResponse {
    pub results: Vec<OtcBalancesRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtcCounterpartyPermissions {
    pub can_trade_with_cpt: bool,
    pub can_report_for_cpt: bool,
    pub can_cpt_report_for_me: bool,
    pub counterparty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtcCounterpartiesResponse {
    pub results: Vec<OtcCounterpartyPermissions>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtcMarket {
    pub instrument: String,
    pub symbol: String,
    pub venue: String,
    pub trading_status: String,
    #[serde_as(as = "Option<TryFromInto<f64>>")]
    pub tick_size: Option<Decimal>,
    #[serde_as(as = "Option<TryFromInto<f64>>")]
    pub step_size: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtcMarketsResponse {
    pub results: Vec<OtcMarket>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtcInstrument {
    pub base_asset: String,
    pub quote_asset: String,
    pub symbol: String,
    pub instrument_type: String,
    pub price_precision: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtcInstrumentsResponse {
    pub results: Vec<OtcInstrument>,
    #[serde(default)]
    pub next_page: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtcAsset {
    pub symbol: String,
    pub description: String,
    #[serde(rename = "type")]
    pub asset_type: String,
    pub precision: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtcAssetsResponse {
    pub results: Vec<OtcAsset>,
    #[serde(default)]
    pub next_page: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskMetrics {
    #[serde(with = "rust_decimal::serde::float")]
    pub equity_usd: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub margin_requirement_usd: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub excess_margin_usd: Decimal,
    pub update_time_utc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RiskMetricsDetails {
    #[serde(with = "rust_decimal::serde::float")]
    pub equity_usd: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub margin_requirement_usd: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub excess_margin_usd: Decimal,
    pub update_time_utc: String,
    #[serde(with = "rust_decimal::serde::float")]
    pub nop_utilization: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PermittedAsset {
    pub asset_symbol: String,
    pub asset_symbol_type: String,
    pub risk_tier: String,
    pub permitted_products: Vec<String>,
    pub asset_class: String,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationRecord {
    pub record_timestamp: String,
    pub event_timestamp: String,
    pub event_id: String,
    pub venue: String,
    pub account: String,
    pub asset_symbol: Option<String>,
    #[serde_as(as = "Option<TryFromInto<f64>>")]
    pub quantity: Option<Decimal>,
    #[serde_as(as = "Option<TryFromInto<f64>>")]
    pub annual_rate: Option<Decimal>,
    #[serde_as(as = "Option<TryFromInto<f64>>")]
    pub funding_cost: Option<Decimal>,
    pub trade_id: Option<String>,
    #[serde_as(as = "Option<TryFromInto<f64>>")]
    pub trade_notional: Option<Decimal>,
    #[serde_as(as = "Option<TryFromInto<f64>>")]
    pub cip_cost: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationsResponse {
    pub results: Vec<CalculationRecord>,
    #[serde(default)]
    pub next_page: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountActivityAccount {
    pub venue: String,
    pub account: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountActivityAccountsResponse {
    pub results: Vec<AccountActivityAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route28Trade {
    pub execution_datetime: String,
    pub side: String,
    #[serde(with = "rust_decimal::serde::float")]
    pub quantity: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub price: Decimal,
    pub liquidity: String,
    pub reporting_for: String,
    pub anonymous: bool,
    pub party: String,
    pub counterparty: String,
    pub reported_datetime: String,
    pub trade_id: String,
    pub market: String,
    pub match_status: String,
    pub match_id: Option<String>,
    pub match_datetime: Option<String>,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route28TradesResponse {
    pub results: Vec<Route28Trade>,
    #[serde(default)]
    pub next_page: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route28PositionAccount {
    pub id: String,
    #[serde(rename = "type")]
    pub account_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route28Position {
    pub instrument: String,
    #[serde(with = "rust_decimal::serde::float")]
    pub quantity: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub avg_opening_price: Decimal,
    pub unrealized_pnl_asset: String,
    #[serde(with = "rust_decimal::serde::float")]
    pub unrealized_pnl: Decimal,
    pub update_timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route28PositionEntry {
    pub account: Route28PositionAccount,
    pub positions: Vec<Route28Position>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route28PositionsResponse {
    pub results: Vec<Route28PositionEntry>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route28FundingRate {
    pub instrument_symbol: String,
    pub instrument_type: String,
    pub instrument_symbol_type: String,
    pub event_timestamp: String,
    #[serde_as(as = "Option<TryFromInto<f64>>")]
    pub swap_price: Option<Decimal>,
    #[serde(with = "rust_decimal::serde::float")]
    pub funding_fee_daily_rate_long: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub funding_fee_daily_rate_short: Decimal,
    pub interest_rate_period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route28FundingRatesResponse {
    pub results: Vec<Route28FundingRate>,
}
