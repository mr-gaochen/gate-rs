use std::collections::BTreeMap;

use anyhow::Result;

use serde_json::Value;

use crate::client::GateClient;

use super::model::ContractInfo;

impl GateClient {
    // 获取单个仓位信息
    // GET /futures/{settle}/positions/{contract}
    pub async fn futures_positions(&self, settle: &str, contract: &str) -> Result<Value> {
        let params: BTreeMap<String, String> = BTreeMap::new();
        Ok(self
            .get::<Value>(
                &format!("/futures/{}/positions/{}", settle, contract),
                &params,
            )
            .await?)
    }

    // 获取合约账户
    // GET /futures/{settle}/accounts
    pub async fn futures_account(&self, settle: &str) -> Result<Value> {
        let params: BTreeMap<String, String> = BTreeMap::new();
        Ok(self
            .get::<Value>(&format!("/futures/{}/accounts", settle), &params)
            .await?)
    }

    // 获取合约信息
    // GET /futures/{settle}/contracts/{contract}
    pub async fn futures_contract(&self, settle: &str, contract: &str) -> Result<ContractInfo> {
        let params: BTreeMap<String, String> = BTreeMap::new();
        Ok(self
            .get::<ContractInfo>(
                &format!("/futures/{}/contracts/{}", settle, contract),
                &params,
            )
            .await?)
    }
}
