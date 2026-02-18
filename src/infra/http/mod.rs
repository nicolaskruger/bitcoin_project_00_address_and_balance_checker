use actix_web::web::ServiceConfig;

pub mod blockchain_controller;
pub mod tx_controller;

pub fn controller_http(app: &mut ServiceConfig) {
    blockchain_controller::blockchain_controller(app);
    tx_controller::tx_controller(app);
}
