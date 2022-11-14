use axum::debug_handler;

#[debug_handler]
pub async fn health() -> String {
    info!("get health");
    "Hello, developer.".to_string()
}

#[debug_handler]
pub async fn shorten(body: String) -> String {
    info!("{:?}",body );
    let uuid = nanoid::nanoid!(8);
    return uuid;
}