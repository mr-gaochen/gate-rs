use serde::{Deserialize, Serialize};

// 合约订单详情
#[derive(Debug, Serialize, Deserialize)]
pub struct FuturesOrder {
    pub id: u64,
    pub user: u64,
    pub create_time: f64,
    pub finish_time: Option<f64>,
    pub finish_as: Option<String>, // e.g., "filled"
    pub status: String,            // e.g., "open", "finished", "cancelled"
    pub contract: String,          // contract name, e.g., "BTC_USDT"
    pub size: i64,                 // can be negative for short
    pub iceberg: i64,              // iceberg quantity if any
    pub price: String,             // limit price as string
    pub close: bool,               // DEPRECATED, use is_close
    pub is_close: bool,            // whether it's a close order
    pub reduce_only: bool,
    pub is_reduce_only: bool,
    pub is_liq: bool, // true if it's a liquidation order
    pub tif: String,  // gtc, ioc, poc, fok
    pub left: i64,    // remaining size
    pub fill_price: String,
    pub text: String,              // custom order note
    pub tkfr: String,              // taker fee rate
    pub mkfr: String,              // maker fee rate
    pub refu: u64,                 // referral user id
    pub auto_size: Option<String>, // can be "close_long", "close_short", or null
    pub stp_id: u64,
    pub stp_act: String, // co, cn, cb, -
    pub amend_text: String,
    pub biz_info: Option<String>,
}
