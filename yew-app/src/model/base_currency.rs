use super::BaseCurrency;

pub fn eth() -> BaseCurrency {
    BaseCurrency {
        name: String::from("Ether"),
        symbol: String::from("ETH"),
        decimals: 18,
    }
}

pub fn avax() -> BaseCurrency {
    BaseCurrency {
        name: String::from("AVAX"),
        symbol: String::from("AVAX"),
        decimals: 18,
    }
}

pub fn anvil() -> BaseCurrency {
    BaseCurrency {
        name: String::from("ANVIL"),
        symbol: String::from("ANVIL"),
        decimals: 18,
    }
}
