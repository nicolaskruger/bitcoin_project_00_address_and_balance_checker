use actix_web::{HttpResponse, Responder, get, web::ServiceConfig};

use crate::infra::repository::blockchain_repository::{
    fetch_blockchain_hash_last, fetch_blockchain_info, fetch_blockchain_last,
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
}
