use actix_web::{HttpResponse, Responder, get, web::ServiceConfig};

use crate::infra::repository::mempool_repository::fetch_mempool;

#[get("/mempool")]
pub async fn mempool() -> impl Responder {
    match fetch_mempool() {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
pub fn mempool_controller(app: &mut ServiceConfig) {
    app.service(mempool);
}
