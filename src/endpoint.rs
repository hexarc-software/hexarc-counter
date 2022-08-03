use actix_web::{get, Responder, HttpResponse};
use actix_web::web::{Data, Query};
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Shield {
    schema_version: i32,
    label: String,
    message: String,
    cache_seconds: Option<u32>,
}

impl Shield {
    pub fn new(label: String, message: String) -> Self {
        Self {
            schema_version: 1,
            label,
            message,
            cache_seconds: Some(300),
        }
    }
}

#[derive(Deserialize)]
pub struct ViewParams {
    name: String,
    label: Option<String>,
}

#[derive(Deserialize)]
pub struct AddViewParams {
    name: String,
}

#[get("/views")]
pub async fn get_view_count(app_state: Data<AppState>, query: Query<ViewParams>) -> impl Responder {
    let ViewParams { name, label } = query.into_inner();
    if name.is_empty() {
        return HttpResponse::BadRequest().finish();
    }

    match app_state.view_service.get_view_count(name.as_str()).await {
        Ok(count) => {
            let label = label.unwrap_or_else(|| String::from("Views"));
            let message = count.to_string();
            let shield = Shield::new(label, message);
            HttpResponse::Ok().json(shield)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/tracker")]
pub async fn add_view(app_state: Data<AppState>, query: Query<AddViewParams>) -> impl Responder {
    const PIXEL: &str = include_str!("pixel.svg");
    const SVG_MIME: &str = "image/svg+xml";
    const CACHE_CONTROL: (&str, &str) = (
        "Cache-Control",
        "max-age=0, no-cache, no-store, must-revalidate",
    );

    let AddViewParams { name } = query.into_inner();
    if name.is_empty() {
        return HttpResponse::BadRequest().finish();
    }

    let _ = app_state.view_service.add_view(name).await;
    HttpResponse::Ok()
        .append_header(CACHE_CONTROL)
        .content_type(SVG_MIME)
        .body(PIXEL)
}
