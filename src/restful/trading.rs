use std::collections::BTreeMap;

use crate::client::GateClient;
use anyhow::Result;
use serde_json::{json, Value};

use super::model::FuturesOrder;

/// 合约交易下单

impl GateClient {
    // 合约交易下单
    // POST /futures/{settle}/orders
    // settle: 结算货币
    // size: 交易数量，正数为买入，负数为卖出。平仓委托则设置为0
    // close: 设置为 true 的时候执行平仓操作，并且size应设置为0
    // 双仓模式下用于设置平仓的方向，close_long 平多头， close_short 平空头，需要同时设置 size 为 0
    // tif: gtc
    pub async fn futures_trade_orders(
        &self,
        settle: &str,
        contract: &str,
        size: i64,
        price: Option<&str>,
        close: Option<bool>,
        auto_size: Option<&str>,
        tif: Option<&str>,
    ) -> Result<FuturesOrder> {
        let mut params: BTreeMap<String, Value> = BTreeMap::new();

        params.insert("contract".into(), json!(contract));
        params.insert("size".into(), json!(size));
        params.insert("iceberg".into(), json!(0));

        //委托价
        if let Some(price) = price {
            params.insert("price".into(), json!(price));
        }

        if let Some(close) = close {
            params.insert("close".into(), json!(close));
        }

        if let Some(auto_size) = auto_size {
            params.insert("auto_size".into(), json!(auto_size));
        }

        if let Some(tif) = tif {
            params.insert("tif".into(), json!(tif));
        }

        Ok(self
            .post::<FuturesOrder>(&format!("/futures/{}/orders", settle), &params)
            .await?)
    }

    // 查询订单详情
    // GET /futures/{settle}/orders/{order_id}
    pub async fn futures_orders(&self, settle: &str, order_id: &str) -> Result<FuturesOrder> {
        let params: BTreeMap<String, String> = BTreeMap::new();
        Ok(self
            .get::<FuturesOrder>(&format!("/futures/{}/orders/{}", settle, order_id), &params)
            .await?)
    }
}
