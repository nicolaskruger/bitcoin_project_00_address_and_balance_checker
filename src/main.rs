use bitcoincore_rpc::{Auth, Client, RpcApi};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc = Client::new(
        "http://127.0.0.1:18332", // testnet
        Auth::UserPass("bitcoinrpc".into(), "supersegredo".into()),
    )?;

    let info = rpc.get_blockchain_info()?;
    println!("Altura atual: {}", info.blocks);

    Ok(())
}
