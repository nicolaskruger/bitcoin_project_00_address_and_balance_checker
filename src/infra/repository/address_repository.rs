use bitcoin::Address;
use bitcoincore_rpc::{
    RpcApi,
    json::{self},
};

use crate::infra::repository::rpc_repository::rpc_client;

pub fn fetch_address(
    address: &Address,
) -> Result<json::GetAddressInfoResult, Box<dyn std::error::Error>> {
    let rpc = rpc_client()?;

    let add_out = rpc.get_address_info(address);

    match add_out {
        Ok(out) => Ok(out),
        Err(_) => Err("not found".into()),
    }
}

#[cfg(test)]
mod tests {
    use bitcoin::Network;

    use super::*;
    use std::str::FromStr;

    #[tokio::test]
    #[ignore]
    async fn fetch_address_test() {
        // cargo test fetch_address_test -- --no-capture --ignored
        let addr = Address::from_str("tb1qdg3akqcrdgjurmgzzggqccxngvnqr3af74mrqq")
            .unwrap()
            .require_network(Network::Testnet)
            .unwrap();

        match fetch_address(&addr) {
            Ok(_) => println!("good"),
            Err(err) => println!("err: {}", err),
        };
    }
}
