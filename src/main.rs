mod endpoint;
mod service;

use std::env;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use mongodb::Client;
use mongodb::options::ClientOptions;
use crate::service::{View, ViewService};

/// The settings to run an application instance.
struct Settings {
    /// The web server port to run on.
    port: u16,
    /// The MongoDB database url.
    database_url: String,
}

impl Settings {
    /// Create an instance of the Settings struct
    /// from the environment variables.
    pub fn from_env() -> Self {
        let port: u16 = env::var("PORT")
            .expect("PORT expected")
            .parse()
            .expect("PORT must be a number");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL expected");
        Self { port, database_url }
    }
}

/// The shareable state for accessing
/// across different parts of the application.
pub struct AppState {
    view_service: ViewService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read the application settings from the env.
    let Settings { port, database_url } = Settings::from_env();

    // Create the database connection for the application.
    let options = ClientOptions::parse(database_url).await.unwrap();
    let client = Client::with_options(options).unwrap();
    let db = client.database("counter_db");
    let view_collection = db.collection::<View>("views");

    // Initialize and start the web server.
    HttpServer::new(move || {
        // Create the shareable state for the application.
        let view_service = ViewService::new(view_collection.clone());
        let app_state = AppState { view_service };

        // Create the application with the shareable state and
        // wired-up API endpoints.
        App::new()
            .app_data(Data::new(app_state))
            .service(endpoint::get_view_count)
            .service(endpoint::add_view)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
