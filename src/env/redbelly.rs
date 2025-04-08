use {
    super::ProviderConfig,
    crate::providers::{Priority, Weight},
    std::collections::HashMap,
};

#[derive(Debug)]
pub struct MonadConfig {
    pub supported_chains: HashMap<String, (String, Weight)>,
}

impl Default for MonadConfig {
    fn default() -> Self {
        Self {
            supported_chains: default_supported_chains(),
        }
    }
}

impl ProviderConfig for MonadConfig {
    fn supported_chains(self) -> HashMap<String, (String, Weight)> {
        self.supported_chains
    }

    fn supported_ws_chains(self) -> HashMap<String, (String, Weight)> {
        HashMap::new()
    }

    fn provider_kind(&self) -> crate::providers::ProviderKind {
        crate::providers::ProviderKind::Redbelly
    }
}

fn default_supported_chains() -> HashMap<String, (String, Weight)> {
    // Keep in-sync with SUPPORTED_CHAINS.md

    HashMap::from([
        // Redbelly testnet
        (
            "eip155:163".into(),
            (
                "https://governors.testnet.redbelly.network".into(),
                Weight::new(Priority::Normal).unwrap(),
            ),
        ),
        // Redbelly mainnet
        (
            "eip155:151".into(),
            (
                "https://governors.mainnet.redbelly.network".into(),
                Weight::new(Priority::Normal).unwrap(),
            ),
        )
    ])
}
