use std::str::FromStr;

use actix_web::{
    HttpResponse, Responder, get,
    web::{self, ServiceConfig},
};
use bitcoin::Txid;
use bitcoincore_rpc::json::GetTxOutResult;

use crate::infra::repository::tx_repository::fetch_tx_by_id;

fn _tx_by_id(tx_str: String) -> Result<GetTxOutResult, Box<dyn std::error::Error>> {
    let txid = Txid::from_str(&tx_str)?;
    fetch_tx_by_id(&txid)
}

#[get("/tx/{tx}")]
pub async fn tx_by_id(path: web::Path<String>) -> impl Responder {
    match _tx_by_id(path.into_inner()) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn tx_controller(app: &mut ServiceConfig) {
    app.service(tx_by_id);
}
