use serde::{Deserialize, Serialize};

// 合约订单详情
#[derive(Debug, Serialize, Deserialize)]
pub struct FuturesOrder {
    pub id: Option<u64>,
    pub user: u64,
    pub create_time: f64,
    pub finish_time: Option<f64>,
    pub finish_as: Option<String>, // e.g., "filled"
    pub status: String,            // e.g., "open", "finished", "cancelled"
    pub contract: String,          // contract name, e.g., "BTC_USDT"
    pub size: i64,                 // can be negative for short
    pub iceberg: i64,              // iceberg quantity if any
    pub price: String,             // limit price as string
    pub close: Option<bool>,       // DEPRECATED, use is_close
    pub is_close: bool,            // whether it's a close order
    pub reduce_only: Option<bool>,
    pub is_reduce_only: bool,
    pub is_liq: bool,        // true if it's a liquidation order
    pub tif: Option<String>, // gtc, ioc, poc, fok
    pub left: Option<i64>,   // remaining size
    pub fill_price: Option<String>,
    pub text: Option<String>,      // custom order note
    pub tkfr: Option<String>,      // taker fee rate
    pub mkfr: Option<String>,      // maker fee rate
    pub refu: Option<u64>,         // referral user id
    pub auto_size: Option<String>, // can be "close_long", "close_short", or null
    pub stp_id: u64,
    pub stp_act: String, // co, cn, cb, -
    pub amend_text: String,
    pub biz_info: String,
}

// 合约信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ContractInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub contract_type: String,
    pub quanto_multiplier: String,
    pub ref_discount_rate: String,
    pub order_price_deviate: String,
    pub maintenance_rate: String,
    pub mark_type: String,
    pub last_price: String,
    pub mark_price: String,
    pub index_price: String,
    pub funding_rate_indicative: String,
    pub mark_price_round: String,
    pub funding_offset: i64,
    pub in_delisting: bool,
    pub risk_limit_base: String,
    pub interest_rate: String,
    pub order_price_round: String,
    pub order_size_min: i64,
    pub ref_rebate_rate: String,
    pub funding_interval: i64,
    pub risk_limit_step: String,
    pub leverage_min: String,
    pub leverage_max: String,
    pub risk_limit_max: String,
    pub maker_fee_rate: String,
    pub taker_fee_rate: String,
    pub funding_rate: String,
    pub order_size_max: i64,
    pub funding_next_apply: i64,
    pub short_users: i64,
    pub config_change_time: i64,
    pub trade_size: i64,
    pub position_size: i64,
    pub long_users: i64,
    pub funding_impact_value: String,
    pub orders_limit: i64,
    pub trade_id: i64,
    pub orderbook_id: i64,
    pub enable_bonus: bool,
    pub enable_credit: bool,
    pub create_time: i64,
    pub funding_cap_ratio: String,
}
