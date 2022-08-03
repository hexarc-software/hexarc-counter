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
}

impl Shield {
    pub fn new(label: String, message: String) -> Self {
        Self {
            schema_version: 1,
            label,
            message,
        }
    }
}

#[derive(Deserialize)]
pub struct ViewParams {
    name: String,
    label: Option<String>,
    accounted: Option<bool>,
}

#[derive(Deserialize)]
pub struct AddViewParams {
    name: String,
}

#[get("/views")]
pub async fn get_view_count(app_state: Data<AppState>, query: Query<ViewParams>) -> impl Responder {
    let ViewParams { name, label, accounted } = query.into_inner();
    if name.is_empty() {
        return HttpResponse::BadRequest().finish();
    }

    if accounted.unwrap_or(false) {
        let _ = app_state.view_service.add_view(name.as_str()).await;
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

    let AddViewParams { name } = query.into_inner();
    if name.is_empty() {
        return HttpResponse::BadRequest().finish();
    }

    let _ = app_state.view_service.add_view(name).await;
    HttpResponse::Ok().content_type(SVG_MIME).body(PIXEL)
}
