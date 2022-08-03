mod endpoint;
mod model;

use std::env;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .expect("PORT expected")
        .parse()
        .expect("PORT must be a number");

    let _database_url = env::var("DATABASE_URL").expect("DATABASE_URL expected");

    HttpServer::new(move || {
        App::new()
            .service(endpoint::get_view_count)
            .service(endpoint::inc_view_count)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
