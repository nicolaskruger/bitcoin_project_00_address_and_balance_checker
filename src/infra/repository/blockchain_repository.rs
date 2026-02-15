use bitcoincore_rpc::{Auth, Client, RpcApi, json::GetBlockchainInfoResult};

pub async fn fetch_last_blockchain() -> Result<GetBlockchainInfoResult, Box<dyn std::error::Error>>
{
    let rpc = Client::new(
        "http://127.0.0.1:18332", // testnet
        Auth::UserPass("bitcoinrpc".into(), "supersegredo".into()),
    )?;

    Ok(rpc.get_blockchain_info()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn fetch_last_blockchain_test() {
        if let Ok(block) = fetch_last_blockchain().await {
            println!("Altura actual: {}", block.blocks);
        }
    }
}
