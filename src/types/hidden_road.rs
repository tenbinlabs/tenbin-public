use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, TryFromInto, serde_as};

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetTransferRequest {
    /// Unique client-generated ID
    pub transfer_id: String,
    /// Asset identifier (e.g. USDT_ERC20 -> /atm/assets/digital)
    pub asset: String,
    /// Namespace for asset identifier (typically 'fireblocks')
    pub asset_namespace: String,
    /// Asset transfer amount
    #[serde_as(as = "DisplayFromStr")]
    pub quantity: Decimal,
    /// Source account
    pub source_exchange_account_name: String,
    /// Source exchange
    pub source_exchange_name: String,
    /// Destination account
    pub destination_exchange_account_name: String,
    /// Destination exchange
    pub destination_exchange_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetTransfer {
    /// Unique client-generated ID
    pub transfer_id: String,
    /// Asset identifier (e.g. USDT_ERC20 -> /atm/assets/digital)
    pub asset: String,
    /// Namespace for asset identifier (typically 'fireblocks')
    pub asset_namespace: String,
    /// Asset transfer amount
    #[serde(with = "rust_decimal::serde::float")]
    pub quantity: Decimal,
    /// Source account
    pub source_exchange_account_name: String,
    /// Source exchange
    pub source_exchange_name: String,
    /// Destination account
    pub destination_exchange_account_name: String,
    /// Destination exchange
    pub destination_exchange_name: String,
    /// Created timestamp
    pub created_datetime: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_datetime: DateTime<Utc>,
    /// Transfer status
    pub status: AtmTransferStatus,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetConversionRequest {
    /// Unique client-generated ID
    pub asset_conversion_id: String,
    /// Conversion quantity
    #[serde_as(as = "DisplayFromStr")]
    pub quantity: Decimal,
    /// Source asset identifier (e.g. USDC)
    pub asset: String,
    /// Destination asset identifier (e.g. USD)
    pub to_asset: String,
    /// Namespace for source asset (typically 'fireblocks')
    pub asset_namespace: String,
    /// Target account name
    pub exchange_account_name: String,
    /// Target exchange name
    pub exchange_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetConversion {
    /// Unique client-generated ID
    pub asset_conversion_id: String,
    /// Conversion quantity
    #[serde(with = "rust_decimal::serde::float")]
    pub quantity: Decimal,
    /// Source asset identifier (e.g. USDC)
    pub asset: String,
    /// Destination asset identifier (e.g. USD)
    pub to_asset: String,
    /// Namespace for source asset (typically 'fireblocks')
    pub asset_namespace: String,
    /// Target account name
    pub exchange_account_name: String,
    /// Target exchange name
    pub exchange_name: String,
    /// Created timestamp
    pub created_datetime: String,
    /// Last updated timestamp
    pub updated_datetime: String,
    /// Conversion status
    pub status: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AtmTransferStatus {
    Pending,
    AwaitingApproval,
    ApprovalRejected,
    Approved,
    Rejected,
    InProgress,
    QueuedForSupport,
    RollbackRequested,
    RollbackInProgress,
    RollbackComplete,
    RetryRequested,
    Failed,
    Complete,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepositRequest {
    /// Unique client-generated ID
    pub transaction_id: String,
    /// Source location ID (use -> /atm/payment_accounts)
    pub source_location_id: String,
    /// Destination location ID (use -> /atm/payment_accounts)
    pub destination_location_id: String,
    /// Asset identifier (e.g. USDC)
    pub asset: String,
    /// Namespace for asset (typically 'fireblocks')
    pub asset_namespace: String,
    /// Deposit amount
    #[serde_as(as = "DisplayFromStr")]
    pub quantity: Decimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Deposit {
    /// Unique client-generated ID
    pub transaction_id: String,
    /// Source location ID (use -> /atm/payment_accounts)
    pub source_location_id: String,
    /// Destination location ID (use -> /atm/payment_accounts)
    pub destination_location_id: String,
    /// Asset identifier (e.g. USDC)
    pub asset: String,
    /// Namespace for asset (typically 'fireblocks')
    pub asset_namespace: String,
    /// Deposit amount
    #[serde(with = "rust_decimal::serde::float")]
    pub quantity: Decimal,
    /// Created timestamp
    pub created_datetime: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_datetime: DateTime<Utc>,
    /// Deposit status
    pub status: AtmTransferStatus,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithdrawalRequest {
    /// Unique client-generated ID
    pub transaction_id: String,
    /// Source location ID (use -> /atm/payment_accounts)
    pub source_location_id: String,
    /// Destination location ID (use -> /atm/payment_accounts)
    pub destination_location_id: String,
    /// Asset identifier (e.g. USDC)
    pub asset: String,
    /// Namespace for asset (typically 'fireblocks')
    pub asset_namespace: String,
    /// Withdrawal amount
    #[serde_as(as = "DisplayFromStr")]
    pub quantity: Decimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Withdrawal {
    /// Unique client-generated ID
    pub transaction_id: String,
    /// Source location ID (use -> /atm/payment_accounts)
    pub source_location_id: String,
    /// Destination location ID (use -> /atm/payment_accounts)
    pub destination_location_id: String,
    /// Asset identifier (e.g. USDC)
    pub asset: String,
    /// Namespace for asset (typically 'fireblocks')
    pub asset_namespace: String,
    /// Withdrawal amount
    #[serde(with = "rust_decimal::serde::float")]
    pub quantity: Decimal,
    /// Created timestamp
    pub created_datetime: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_datetime: DateTime<Utc>,
    /// Withdrawal status
    pub status: AtmTransferStatus,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtcReportTradeRequest {
    /// Execution timestamp of OTC trade
    pub execution_datetime: String,
    /// Side of trade (buy or sell)
    pub side: String,
    /// Trade amount
    pub quantity: Decimal,
    /// Trade price
    pub price: Decimal,
    /// Indicates if the party was a maker or taker
    pub liquidity: String,
    /// Indicates the reporting party (party, counterparty, or both)
    pub reporting_for: String,
    /// Whether the trade is anonymous
    pub anonymous: bool,
    /// Identifier for the party
    pub party: String,
    /// Identifier for the counterparty
    pub counterparty: String,
    /// Unique client-generated ID
    pub trade_id: String,
    /// The market symbol (e.g. BTCUSD_HRP)
    pub market: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtcTrade {
    /// Execution timestamp of OTC trade
    pub execution_datetime: String,
    /// Side of trade (buy or sell)
    pub side: String,
    /// Trade amount
    #[serde(with = "rust_decimal::serde::float")]
    pub quantity: Decimal,
    /// Trade price
    #[serde(with = "rust_decimal::serde::float")]
    pub price: Decimal,
    /// Indicates if the party was a maker or taker
    pub liquidity: String,
    /// Indicates the reporting party (party, counterparty, or both)
    pub reporting_for: String,
    /// Whether the trade is anonymous
    pub anonymous: bool,
    /// Identifier for the party
    pub party: String,
    /// Identifier for the counterparty
    pub counterparty: String,
    /// Reported timestamp
    pub reported_datetime: String,
    /// Unique client-generated ID
    pub trade_id: String,
    /// The market symbol (e.g. BTCUSD_HRP)
    pub market: String,
    /// Current matching status (e.g. 'pending')
    pub match_status: String,
    /// Unique ID – if matched
    pub match_id: Option<String>,
    /// Match timestamp – if matched
    pub match_datetime: Option<String>,
    /// Additional info
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtcTradesResponse {
    /// Array of OTC trades
    pub results: Vec<OtcTrade>,
    /// Cursor for next page
    #[serde(default)]
    pub next_page: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtcReportTradeResponse {
    /// Success / failure
    pub success: bool,
    /// Additional info
    pub message: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionsOrBalancesEntry {
    /// Instrument name
    pub instrument: String,
    /// Symbol type
    pub instrument_symbol_type: String,
    /// Instrument type (e.g. swap)
    pub instrument_type: String,
    /// Position size
    #[serde(with = "rust_decimal::serde::float")]
    pub quantity: Decimal,
    /// Asset used for P&L
    pub unrealized_pnl_asset: Option<String>,
    /// Unrealized P&L
    #[serde_as(as = "Option<TryFromInto<f64>>")]
    pub unrealized_pnl: Option<Decimal>,
    /// Average entry price
    #[serde_as(as = "Option<TryFromInto<f64>>")]
    pub average_entry_price: Option<Decimal>,
    /// Mark price
    #[serde_as(as = "Option<TryFromInto<f64>>")]
    pub marking_price: Option<Decimal>,
    /// Settlement timestamp (if available)
    pub settlement_timestamp: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionsOrBalancesRecord {
    /// Ledger insert timestamp
    pub record_timestamp: String,
    /// Event timestamp
    pub event_timestamp: String,
    /// Unique event ID
    pub event_id: String,
    /// Version of corrected record
    pub correction_version: i32,
    /// Reason for correction
    pub correction_reason: Option<String>,
    /// If true, then record is logically deleted (see -> 'Introduction to HRP API')
    pub is_deleted: bool,
    /// Venue identifier
    pub venue: String,
    /// Account reference (<SHORTCODE>:ACCOUNT_TYPE)
    pub account: String,
    /// Unused for positions snapshot
    pub event_sub_type: Option<String>,
    /// Array of instruments
    pub positions_or_balances: Vec<PositionsOrBalancesEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionsOrBalancesResponse {
    /// Array of position snapshots
    pub results: Vec<PositionsOrBalancesRecord>,
    /// Cursor for next page
    #[serde(default)]
    pub next_page: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferRecord {
    pub record_timestamp: String,
    pub event_timestamp: String,
    pub event_id: String,
    pub venue: String,
    pub account: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub asset: String,
    #[serde(with = "rust_decimal::serde::float")]
    pub quantity: Decimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransfersResponse {
    pub results: Vec<TransferRecord>,
    #[serde(default)]
    pub next_page: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalAssetsResponse {
    pub assets: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentAccount {
    pub id: String,
    pub name: String,
    pub approval_status: Option<String>,
    pub wallet_address: Option<String>,
    pub destination_tag: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub account_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradesResponse {
    pub results: Vec<Trade>,
    #[serde(default)]
    pub next_page: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trade {
    pub record_timestamp: String,
    pub event_timestamp: String,
    pub event_id: String,
    pub correction_version: i32,
    pub correction_reason: Option<String>,
    pub is_deleted: bool,
    pub venue: String,
    pub account: String,
    pub event_sub_type: Option<String>,
    pub market: String,
    pub market_symbol_type: String,
    pub market_type: String,
    pub trade_id: String,
    pub order_id: Option<String>,
    pub liquidity: String,
    pub side: String,
    #[serde(with = "rust_decimal::serde::float")]
    pub quantity: Decimal,
    #[serde(with = "rust_decimal::serde::float")]
    pub price: Decimal,
    pub contra_party: Option<String>,
    #[serde_as(as = "Option<TryFromInto<f64>>")]
    pub quote_quantity: Option<Decimal>,
    pub settlement_timestamp: Option<String>,
    pub execution_venue: String,
    pub multi_leg_trade_parent_id: Option<String>,
    pub linked_trade_event_id: Option<String>,
}

/// Generic HRP operation request
#[derive(Debug, Clone, PartialEq)]
pub enum HrpOperationRequest {
    Withdrawal(WithdrawalRequest),
    Deposit(DepositRequest),
    Transfer(AssetTransferRequest),
}
