use super::utils::de_float_from_str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KlineData {
    pub t: i64, // 	时间
    pub v: f64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub c: f64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub h: f64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub l: f64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub o: f64,
    #[serde(deserialize_with = "de_float_from_str")]
    pub a: f64, // 成交量
    pub n: String, // 合约名称
}

impl KlineData {
    fn new() {
        
    }
}