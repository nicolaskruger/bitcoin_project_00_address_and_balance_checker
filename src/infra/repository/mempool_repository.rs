use bitcoincore_rpc::{
    RpcApi,
    json::{self},
};

use crate::infra::repository::rpc_repository::rpc_client;

pub fn fetch_mempool() -> Result<json::GetMempoolInfoResult, Box<dyn std::error::Error>> {
    let rpc = rpc_client()?;

    let mempool = rpc.get_mempool_info();

    match mempool {
        Ok(out) => Ok(out),
        Err(_) => Err("not found".into()),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn fetch_mempool_test() {
        // cargo test fetch_mempool_test -- --no-capture --ignored
        let mempool = fetch_mempool();

        match mempool {
            Ok(_) => println!("good"),
            Err(err) => println!("err: {}", err),
        };
    }
}
