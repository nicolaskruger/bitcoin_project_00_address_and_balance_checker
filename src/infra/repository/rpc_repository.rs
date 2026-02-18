use bitcoincore_rpc::{Auth, Client};

use crate::value_objects::env_variables::EnvVariables;

pub fn rpc_client() -> Result<Client, Box<dyn std::error::Error>> {
    let env = EnvVariables::new();

    let rpc = Client::new(
        &env.blockchain_uri(),
        Auth::UserPass(env.blockchain_user(), env.blockchain_password()),
    )?;

    Ok(rpc)
}
