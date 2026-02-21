use actix_web::web::ServiceConfig;

pub mod address_controller;
pub mod blockchain_controller;
pub mod mempool_controller;
pub mod tx_controller;

pub fn controller_http(app: &mut ServiceConfig) {
    blockchain_controller::blockchain_controller(app);
    tx_controller::tx_controller(app);
    address_controller::address_controller(app);
    mempool_controller::mempool_controller(app);
}
