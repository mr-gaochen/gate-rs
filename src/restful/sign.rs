use anyhow::Result;
use hmac_sha512::Hash;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;

use crate::client::GateClient;

impl GateClient {
    pub async fn get<T>(
        &self,
        request_path: &str,
        parameters: &BTreeMap<String, String>,
    ) -> Result<T>
    where
        T: DeserializeOwned + std::fmt::Debug,
    {
        let timestamp = self.get_timestamp();
        // 构建签名字符串
        let query_str = Self::build_query_string(parameters);
        let pre_sign = format!(
            "{}{}{}{}{}",
            "GET",
            format!("{}/{}", self.prefix, request_path),
            query_str,
            "",
            timestamp
        );
        let sign = self.sha512_hex(&pre_sign);

        let url = self.build_full_url(request_path, parameters);

        let headers = self.create_header(&sign, &timestamp);
        if self.debug {
            println!("FIRST_SIGN:{}", pre_sign.clone());
            println!("[GET] URL: {}", url);
            println!("[GET] Params: {:?}", parameters);
            println!("[GET] Sign: {}", sign);
        }

        let client = reqwest::Client::new();
        let resp = client
            .get(&url)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        if self.debug {
            println!("[GET] Response: {:#?}", resp);
        }

        Ok(serde_json::from_str::<T>(&resp)?)
    }

    // 获取时间戳
    fn get_timestamp(&self) -> String {
        chrono::Utc::now().timestamp_millis().to_string()
    }

    fn sha512_hex(&self, input: &str) -> String {
        let mut hasher = Hash::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    fn create_header(&self, sign: &str, timestamp: &str) -> HeaderMap {
        // 处理请求头 headers
        let mut header_map = HeaderMap::new();
        header_map.insert("Accept", HeaderValue::from_str("application/json").unwrap());
        header_map.insert("Content-Type", HeaderValue::from_static("application/json"));
        header_map.insert("SIGN", HeaderValue::from_str(&sign).unwrap());
        header_map.insert("Timestamp", HeaderValue::from_str(&timestamp).unwrap());
        header_map.insert("KEY", HeaderValue::from_str(&self.api_key).unwrap());
        header_map
    }

    /// 构建 query 参数的签名字符串（key+value 按照 ASCII 排序）
    fn build_query_string(params: &BTreeMap<String, String>) -> String {
        params
            .iter()
            .map(|(k, v)| format!("{}{}", k, v))
            .collect::<Vec<_>>()
            .join("&")
    }

    /// 构建完整 URL（含 query 参数）
    fn build_full_url(&self, path: &str, params: &BTreeMap<String, String>) -> String {
        let domain = self.domain.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        let prefix = self.prefix.trim_start_matches('/');
        if params.is_empty() {
            format!("{}/{}/{}", domain, prefix, path)
        } else {
            let query_string = params
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            format!("{}/{}?{}", domain, path, query_string)
        }
    }
}
