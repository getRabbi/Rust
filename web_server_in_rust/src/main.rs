use std::io;

use actix_web::*;

#[get("/")]
async fn connect() -> impl Responder {
    HttpResponse::Ok().body("Server is running")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(connect))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
