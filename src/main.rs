mod endpoint;
mod model;

use std::env;
use actix_web::{App, HttpServer};
use endpoint::{ping, point};

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
