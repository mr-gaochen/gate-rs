use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GateClient {
    pub debug: bool,
    pub testnet: bool,
    pub api_key: String,
    pub secret_key: String,
    pub domain: String,
    pub prefix: String,
}

impl GateClient {
    pub fn new(
        debug: bool,
        testnet: bool,
        api_key: impl Into<String>,
        secret_key: impl Into<String>,
        domain: impl Into<String>,
        prefix: impl Into<String>,
    ) -> Self {
        GateClient {
            debug,
            testnet,
            api_key: api_key.into(),
            secret_key: secret_key.into(),
            domain: domain.into(),
            prefix: prefix.into(),
        }
    }
}
