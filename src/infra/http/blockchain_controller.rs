use std::str::FromStr;

use actix_web::{
    HttpResponse, Responder, get,
    web::{self, ServiceConfig},
};
use bitcoincore_rpc::bitcoin::{Block, BlockHash};

use crate::infra::repository::blockchain_repository::{
    fetch_blockchain_by_hash, fetch_blockchain_hash_last, fetch_blockchain_info,
    fetch_blockchain_last,
};

#[get("/blockchain/hash/tail")]
pub async fn blockchain_hash_tail() -> impl Responder {
    let fetch = fetch_blockchain_hash_last().await;

    match fetch {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/blockchain/tail")]
pub async fn blockchain_tail() -> impl Responder {
    let fetch = fetch_blockchain_last().await;

    match fetch {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn _blockchain_by_hash(hash: String) -> Result<Block, Box<dyn std::error::Error>> {
    let hash = BlockHash::from_str(&hash)?;
    fetch_blockchain_by_hash(hash).await
}

#[get("/blockchain/{hash}")]
pub async fn blockchain_by_hash(path: web::Path<String>) -> impl Responder {
    let hash = path.into_inner();

    let fetch = _blockchain_by_hash(hash).await;

    match fetch {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/blockchain/info")]
pub async fn blockchain_info() -> impl Responder {
    let fetch = fetch_blockchain_info().await;

    match fetch {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn blockchain_controller(app: &mut ServiceConfig) {
    app.service(blockchain_tail);
    app.service(blockchain_info);
    app.service(blockchain_hash_tail);
    app.service(blockchain_by_hash);
}
