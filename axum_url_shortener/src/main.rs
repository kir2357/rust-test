mod router;

use axum::{routing::{ get, post }, Router};
use log::info;
use std::env;

use router::*;
mod lib {
    pub mod initialize;
}
use lib::initialize::initialize;


#[tokio::main]
async fn main() {
    initialize();

    let _database_url = env::var("DATABASE_URL").expect("環境変数が取得できません");
    
    let log_output_path = env::var("LOG_PATH").expect("環境変数が取得できません");

    println!("{:?}", log_output_path);

    println!("Server started.");
    info!("Server started.");

    let app = Router::new()
       .route("/", get(health)) 
       .route("/shorten", post(shorten));

    axum::Server::bind(&"127.0.0.1:4000".parse().unwrap())
       .serve(app.into_make_service())
       .await
       .unwrap();
}

#[tokio::main]
async fn _1_main() {
    env_logger::init();
    println!("Server started.");

    let app = Router::new().route("/", get(health));

    axum::Server::bind(&"127.0.0.1:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}