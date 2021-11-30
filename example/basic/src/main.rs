mod handlers;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handlers::index))
            .route("/{name}", web::get().to(handlers::greet))
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}
