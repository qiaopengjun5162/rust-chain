use std::sync::Arc;

use ethers::{
    contract::{ContractError, abigen},
    providers::{Http, Provider},
    types::{Address, U256},
};

abigen!(CounterContract, "./src/counter.json");

#[derive(Debug, Clone)]
pub struct Counter {
    pub client: Arc<Provider<Http>>,
    pub contract: CounterContract<Provider<Http>>,
}

impl Counter {
    pub fn new(provider: Arc<Provider<Http>>, address: Address) -> Self {
        let contract = CounterContract::new(address, provider.clone());
        Self {
            client: provider,
            contract,
        }
    }

    pub async fn get_number(&self) -> Result<U256, ContractError<Provider<Http>>> {
        let number = self.contract.number().call().await?;
        Ok(number)
    }
}
