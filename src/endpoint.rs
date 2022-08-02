use actix_web::{get, web, Responder, HttpResponse};
use actix_web::web::{Path, Query};
use serde::Deserialize;
use crate::model::{Shield};

#[derive(Deserialize)]
pub struct ViewParams {
    pub user: String,
    pub label: Option<String>,
}

#[get("/views")]
pub async fn get_view_count(query: Query<ViewParams>) -> impl Responder {
    let ViewParams { user, label } = query.into_inner();
    println!("{}", user);

    let label = label.unwrap_or_else(|| String::from("Views"));
    let message = String::from("0");
    let shield = Shield::new(label, message);

    web::Json(shield)
}

#[get("/tracker/{user}")]
pub async fn inc_view_count(path: Path<String>) -> impl Responder {
    const PIXEL: &str = include_str!("pixel.svg");
    const SVG_MIME: &str = "image/svg+xml";

    let user = path.into_inner();
    println!("User: {}", user);

    HttpResponse::Ok().content_type(SVG_MIME).body(PIXEL)
}
