use bitcoincore_rpc::{
    Auth, Client, RpcApi,
    bitcoin::{Block, BlockHash},
    json::GetBlockchainInfoResult,
};

use crate::value_objects::env_variables::EnvVariables;

fn rcp_client() -> Result<Client, Box<dyn std::error::Error>> {
    let env = EnvVariables::new();

    let rpc = Client::new(
        &env.blockchain_uri(),
        Auth::UserPass(env.blockchain_user(), env.blockchain_password()),
    )?;

    Ok(rpc)
}

pub async fn fetch_blockchain_info() -> Result<GetBlockchainInfoResult, Box<dyn std::error::Error>>
{
    let rpc = rcp_client()?;

    Ok(rpc.get_blockchain_info()?)
}

pub async fn fetch_blockchain_last() -> Result<Block, Box<dyn std::error::Error>> {
    let rpc = rcp_client()?;

    let heitch = rpc.get_block_count()?;
    let block_hash = rpc.get_block_hash(heitch)?;

    let block = rpc.get_block(&block_hash)?;

    Ok(block)
}

pub async fn fetch_blockchain_hash_last() -> Result<BlockHash, Box<dyn std::error::Error>> {
    let rpc = rcp_client()?;

    let heitch = rpc.get_block_count()?;
    let block_hash = rpc.get_block_hash(heitch)?;

    Ok(block_hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn fetch_last_blockchain_test() {
        if let Ok(block) = fetch_blockchain_info().await {
            println!("Altura actual: {}", block.blocks);
        }
    }
}
