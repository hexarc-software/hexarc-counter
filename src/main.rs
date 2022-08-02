use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::env;

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
    let port: u16 = env::var("PORT")
        .expect("PORT expected")
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| App::new().service(ping).service(point))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
