use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};

mod model;
use model::Point;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[get("/point")]
async fn point() -> impl Responder {
    let point = Point::new_random();
    web::Json(point)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(ping).service(point))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
