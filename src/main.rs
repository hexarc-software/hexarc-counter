mod service;
mod endpoint;

use std::env;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use mongodb::Client;
use mongodb::options::ClientOptions;
use crate::service::{View, ViewService};

struct Settings {
    port: u16,
    database_url: String,
}

impl Settings {
    pub fn from_env() -> Self {
        let port: u16 = env::var("PORT")
            .expect("PORT expected")
            .parse()
            .expect("PORT must be a number");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL expected");
        Self { port, database_url }
    }
}

pub struct AppState {
    view_service: ViewService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let Settings { port, database_url } = Settings::from_env();

    let options = ClientOptions::parse(database_url).await.unwrap();
    let client = Client::with_options(options).unwrap();
    let db = client.database("counter_db");
    let view_collection = db.collection::<View>("views");

    HttpServer::new(move || {
        let view_service = ViewService::new(view_collection.clone());
        let app_state = AppState { view_service };

        App::new()
            .app_data(Data::new(app_state))
            .service(endpoint::get_view_count)
            .service(endpoint::add_view)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
