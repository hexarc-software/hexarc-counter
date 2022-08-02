use actix_web::{get, web, HttpResponse, Responder};
use crate::model::Point;

#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[get("/point")]
pub async fn point() -> impl Responder {
    let point = Point::new_random();
    web::Json(point)
}
