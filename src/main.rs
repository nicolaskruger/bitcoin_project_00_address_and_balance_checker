use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
// use bitcoincore_rpc::{Auth, Client, RpcApi};

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let rpc = Client::new(
//         "http://127.0.0.1:18332", // testnet
//         Auth::UserPass("bitcoinrpc".into(), "supersegredo".into()),
//     )?;
//
//     let info = rpc.get_blockchain_info()?;
//     println!("Altura actual: {}", info.blocks);
//
//     Ok(())
// }

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
