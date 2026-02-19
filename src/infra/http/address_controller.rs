use std::str::FromStr;

use actix_web::{
    HttpResponse, Responder, get,
    web::{self, ServiceConfig},
};
use bitcoin::{Address, Network};

use crate::infra::repository::address_repository::fetch_address;

fn _address_by_address(address: &str) -> Address {
    Address::from_str(address)
        .unwrap()
        .require_network(Network::Testnet)
        .unwrap()
}

#[get("/address/{address}")]
pub async fn address_by_address(address: web::Path<String>) -> impl Responder {
    let _addr = _address_by_address(&address.into_inner());

    match fetch_address(&_addr) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
pub fn address_controller(app: &mut ServiceConfig) {
    app.service(address_by_address);
}
