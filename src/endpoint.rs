use actix_web::{get, web, Responder, HttpResponse};
use crate::model::{Shield};

#[get("/views")]
pub async fn get_view_count() -> impl Responder {
    let label = String::from("Profile views");
    let message = String::from("0");
    let shield = Shield::new(label, message);
    web::Json(shield)
}

#[get("/tracker.svg")]
pub async fn inc_view_count() -> impl Responder {
    const PIXEL: &str = include_str!("pixel.svg");
    const SVG_MIME: &str = "image/svg+xml";

    HttpResponse::Ok()
        .content_type(SVG_MIME)
        .body(PIXEL)
}
