use bitcoincore_rpc::{RpcApi, json::GetTxOutResult};

use crate::infra::repository::rpc_repository::rpc_client;

pub fn fetch_tx_by_id(txid: &bitcoin::Txid) -> Result<GetTxOutResult, Box<dyn std::error::Error>> {
    let rpc = rpc_client()?;

    let tx_out = rpc.get_tx_out(txid, 0, None)?;

    match tx_out {
        Some(out) => Ok(out),
        None => Err("not found".into()),
    }
}

#[cfg(test)]
mod tests {
    use bitcoin::Txid;

    use super::*;
    use std::str::FromStr;

    #[tokio::test]
    #[ignore]
    async fn tx_by_id_test() {
        // cargo test tx_by_id_test -- --no-capture --ignored
        let tx = Txid::from_str("4a1392e70bde7f36181955ff09447567cbddce6478ed55788a779bf9cf60d8e3")
            .unwrap();

        match fetch_tx_by_id(&tx) {
            Ok(_) => println!("good"),
            Err(err) => println!("err: {}", err),
        };
    }
}
