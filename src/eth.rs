//! Ethereum interface

use std::sync::Arc;
use std::time::Duration;

use ethers::contract::abigen;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::providers::{JsonRpcClient, Provider};
use ethers::signers::{coins_bip39::English, MnemonicBuilder};
use ethers::signers::{Signer, Wallet};

abigen!(
    CidStorage,
    "solidity/artifacts/CidStorage.sol/CidStorage.json",
);

#[derive(Debug)]
pub struct Contract {
    cid: String,
    eth_api_address: String,
    mnemonic: String,
    password: Option<String>,
}

impl Contract {
    /// Constructor.
    pub fn new(
        cid: String,
        eth_api_address: String,
        mnemonic: String,
        password: Option<String>,
    ) -> Self {
        Self {
            cid,
            eth_api_address,
            mnemonic,
            password,
        }
    }

    /// Evaluate the contract and return CID.
    ///
    /// It's obvious we can just return `self.cid` here, but we should use contracts (i.e. store the
    /// value inside). So we just show that it's stored correctly.
    pub async fn evaluate(&self) -> anyhow::Result<String> {
        let provider =
            Provider::try_from(&self.eth_api_address)?.interval(Duration::from_millis(10));
        let chain_id = chain_id(&provider).await?;
        let wallet = init_wallet(self.mnemonic.as_str(), self.password.clone(), chain_id)?;
        let contract = self.deploy(provider.clone(), wallet).await?;

        contract.set(self.cid.clone()).send().await?.await?;

        Ok(contract.get().call().await?)
    }

    async fn deploy<P: JsonRpcClient + 'static>(
        &self,
        provider: Provider<P>,
        wallet: Wallet<SigningKey>,
    ) -> anyhow::Result<CidStorage<SignerMiddleware<Provider<P>, Wallet<SigningKey>>>> {
        let client = SignerMiddleware::new(provider, wallet);
        let client = Arc::new(client);

        Ok(CidStorage::deploy(Arc::clone(&client), ())?.send().await?)
    }
}

async fn chain_id<P: JsonRpcClient>(provider: &Provider<P>) -> anyhow::Result<u64> {
    let chain_id: String = provider.request("eth_chainId", ()).await?;
    let chain_id = chain_id.trim_start_matches("0x");
    u64::from_str_radix(&chain_id, 16).map_err(anyhow::Error::msg)
}

fn init_wallet(
    mnemonic: &str,
    password: Option<String>,
    chain_id: u64,
) -> anyhow::Result<Wallet<SigningKey>> {
    let wallet = MnemonicBuilder::<English>::default().phrase(mnemonic);
    Ok(password
        .map_or(wallet.clone(), |pass| wallet.password(&pass))
        .build()?
        .with_chain_id(chain_id))
}
