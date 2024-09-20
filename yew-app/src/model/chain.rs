use std::{collections::HashMap, sync::LazyLock};

use super::{base_currency, Chain};



pub fn ethereum() -> Chain {
    Chain {
        name: "Ethereum".into(),
        id: "1".into(),
        rpc_urls: [String::from("https://api.avax-test.network/ext/bc/C/rpc")],
        native_currency: base_currency::eth(),
        block_explorer_urls: [String::from("https://api.avax-test.network/ext/bc/C/rpc")],
    }
}

pub fn avalanche_testnet() -> Chain {
    Chain {
        name: String::from("Avalanche Fuji Testnet"),
        id: String::from("43113"),
        rpc_urls: [String::from("https://api.avax-test.network/ext/bc/C/rpc")],
        native_currency: base_currency::avax(),
        block_explorer_urls: [String::from("https://api.avax-test.network/ext/bc/C/rpc")],
    }
}

pub fn anvil_localnet() -> Chain {
    Chain {
        name: String::from("Anvil Localnet"),
        id: String::from("31337"), 
        rpc_urls: [String::from("http://127.0.0.1:8545")],
        native_currency: base_currency::anvil(),
        block_explorer_urls: [String::from("http://127.0.0.1:8545/ext/bc/C/rpc")],
    }
}

pub const CHAINS: LazyLock<HashMap<String, Chain>> = LazyLock::new(|| {
    vec![ avalanche_testnet(), anvil_localnet()]
    .into_iter()
    .map(|chain| (chain.id.clone(), chain))
    .collect()
});