use axum::{response, routing::{self, get}, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(meta::version()));
}

mod dao;
mod mq;
mod api;