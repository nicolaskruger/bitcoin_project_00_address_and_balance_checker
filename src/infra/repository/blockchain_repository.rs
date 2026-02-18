use bitcoincore_rpc::{
    RpcApi,
    bitcoin::{Block, BlockHash},
    json::GetBlockchainInfoResult,
};

use crate::infra::repository::rpc_repository::rpc_client;

pub async fn fetch_blockchain_info() -> Result<GetBlockchainInfoResult, Box<dyn std::error::Error>>
{
    let rpc = rpc_client()?;

    Ok(rpc.get_blockchain_info()?)
}

pub async fn fetch_blockchain_last() -> Result<Block, Box<dyn std::error::Error>> {
    let rpc = rpc_client()?;

    let heitch = rpc.get_block_count()?;
    let block_hash = rpc.get_block_hash(heitch)?;

    let block = rpc.get_block(&block_hash)?;

    Ok(block)
}

pub async fn fetch_blockchain_hash_last() -> Result<BlockHash, Box<dyn std::error::Error>> {
    let rpc = rpc_client()?;

    let heitch = rpc.get_block_count()?;
    let block_hash = rpc.get_block_hash(heitch)?;

    Ok(block_hash)
}

pub async fn fetch_blockchain_by_hash(
    block_hash: BlockHash,
) -> Result<Block, Box<dyn std::error::Error>> {
    let rpc = rpc_client()?;

    Ok(rpc.get_block(&block_hash)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[tokio::test]
    #[ignore]
    async fn fetch_last_blockchain_test() {
        if let Ok(block) = fetch_blockchain_info().await {
            println!("Altura actual: {}", block.blocks);
        }
    }

    #[tokio::test]
    #[ignore]
    async fn fetch_blockchain_by_hash_test() {
        // cargo test fetch_blockchain_by_hash_test -- --no-capture --ignored
        if let Ok(hash) =
            BlockHash::from_str("0000000000daa121e56a718458fab186c937e8eff99285ac91bbb65b782f4a59")
        {
            let result = fetch_blockchain_by_hash(hash).await;

            match result {
                Ok(_) => println!("good"),
                Err(err) => println!("{}", err),
            }
        }
    }
}
