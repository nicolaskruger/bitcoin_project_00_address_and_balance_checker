use actix_web::{HttpResponse, Responder, get, web::ServiceConfig};

#[get("/blockchain/tail")]
pub async fn blockchain_tail() -> impl Responder {
    HttpResponse::Ok().body("blockchain_tail")
}

pub fn blockchain_controller(app: &mut ServiceConfig) {
    app.service(blockchain_tail);
}
