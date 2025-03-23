use axum::{response::IntoResponse, routing::get, Router, extract::Path};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/hey/:name", get(hey))
        .route("/are-you-ready", get(are_you_ready));

    let port: u16 = std::env::var("PORT")
        .unwrap_or("3000".into())
        .parse()
        .expect("failed to convert to number");

    let ipv6 = SocketAddr::from(([0,0,0,0,0,0,0,0], port));
    let ipv6_listener = TcpListener::bind(&ipv6).await.unwrap();

    tracing::info!("Listening on IPv6 at {}!", ipv6);

    axum::serve(ipv6_listener, app)
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    "Hello, World!"
}

async fn hey(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello, {}!", name)
}

async fn are_you_ready() -> impl IntoResponse {
    "I'm ready!"
}
