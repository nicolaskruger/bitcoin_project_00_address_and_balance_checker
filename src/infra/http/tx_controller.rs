use actix_web::{
    HttpResponse, Responder, get,
    web::{self, ServiceConfig},
};

#[get("/tx/{tx}")]
pub async fn tx_by_id(path: web::Path<String>) -> impl Responder {
    let result = format!("tx: {}", path.into_inner());

    HttpResponse::Ok().json(result)
}

pub fn tx_controller(app: &mut ServiceConfig) {
    app.service(tx_by_id);
}
