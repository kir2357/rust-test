#[macro_use]
extern crate log;

use axum::{routing::{ get, post }, Router};

mod initialize;
use initialize::CONFIG as CONFIG;

mod router;

#[tokio::main]
async fn main() {
    initialize::init();

    let app = Router::new()
        .route("/", get(router::health)) 
        .route("/shorten", post(router::shorten));

    info!("Server start.");

    axum::Server::bind(&CONFIG.url.parse().unwrap())
       .serve(app.into_make_service())
       .await
       .unwrap();
    
}

