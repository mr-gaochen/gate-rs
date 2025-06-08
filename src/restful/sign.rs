use anyhow::Result;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use serde_json::Value;
use sha2::{Digest, Sha512};
use std::collections::BTreeMap;

use crate::client::GateClient;

type HmacSha512 = Hmac<Sha512>;

impl GateClient {
    pub async fn get<T>(
        &self,
        request_path: &str,
        parameters: &BTreeMap<String, String>,
    ) -> Result<T>
    where
        T: DeserializeOwned + std::fmt::Debug,
    {
        let method = "GET";
        let timestamp = self.get_timestamp();
        let url_path = format!(
            "/{}/{}",
            self.prefix.trim_start_matches('/'),
            request_path.trim_start_matches('/')
        );

        // 拼 query string，注意不 URL encode，顺序和请求里一致
        let query_string = if parameters.is_empty() {
            "".to_string()
        } else {
            parameters
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&")
        };

        // GET 请求没有 body，payload 为空字符串，计算 SHA512 hash
        let payload_hash = Self::sha512_hex("");

        // 构造签名字符串
        let sign_str = format!(
            "{}\n{}\n{}\n{}\n{}",
            method, url_path, query_string, payload_hash, timestamp
        );

        let sign = self.hmac_sha512_hex(&sign_str);

        let url = self.build_full_url(request_path, parameters);

        let headers = self.create_header(&sign, &timestamp);
        if self.debug {
            println!("SIGN STRING:\n{}", sign_str);
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

    pub async fn post<T>(
        &self,
        request_path: &str,
        body_params: &BTreeMap<String, Value>,
    ) -> Result<T>
    where
        T: DeserializeOwned + std::fmt::Debug,
    {
        let method = "POST";
        let timestamp = self.get_timestamp();
        let url_path = format!(
            "/{}/{}",
            self.prefix.trim_start_matches('/'),
            request_path.trim_start_matches('/')
        );

        // 构造 JSON body 字符串
        let body_json = serde_json::to_string(body_params)?;
        let payload_hash = Self::sha512_hex(&body_json);

        // 构造签名字符串
        let sign_str = format!(
            "{}\n{}\n{}\n{}\n{}",
            method, url_path, "", payload_hash, timestamp
        );

        let sign = self.hmac_sha512_hex(&sign_str);
        let url = format!(
            "{}/{}/{}",
            self.domain.trim_end_matches('/'),
            self.prefix.trim_start_matches('/'),
            request_path.trim_start_matches('/')
        );
        let headers = self.create_header(&sign, &timestamp);

        if self.debug {
            println!("SIGN STRING:\n{}", sign_str);
            println!("[POST] URL: {}", url);
            println!("[POST] Body: {}", body_json);
            println!("[POST] Sign: {}", sign);
        }

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .headers(headers)
            .body(body_json)
            .send()
            .await?
            .text()
            .await?;

        if self.debug {
            println!("[POST] Response: {:#?}", resp);
        }

        Ok(serde_json::from_str::<T>(&resp)?)
    }

    // 获取时间戳
    fn get_timestamp(&self) -> String {
        chrono::Utc::now().timestamp().to_string()
    }

    // 计算 SHA512 hex
    fn sha512_hex(input: &str) -> String {
        let mut hasher = Sha512::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    // 计算 HMAC-SHA512 hex，用 self.api_secret 作为 key
    fn hmac_sha512_hex(&self, input: &str) -> String {
        let mut mac = HmacSha512::new_from_slice(self.secret_key.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(input.as_bytes());
        let result = mac.finalize().into_bytes();
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
            format!("{}/{}/{}?{}", domain, prefix, path, query_string)
        }
    }
}
