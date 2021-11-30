use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");

    HttpResponse::Ok().json(User {
        name: name.to_string(),
    })
}
