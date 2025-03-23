use axum::{response::IntoResponse, routing::get, Router, extract::Path};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tower::ServiceBuilder;
use axum::http::StatusCode;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(hello))
        .route("/:name", get(hey))
        .route("/are-you-ready", get(are_you_ready))
        .fallback(fallback_404)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
        );

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
    println!("Hello, World!");
    "Hello, World!"
}

async fn hey(Path(name): Path<String>) -> impl IntoResponse {
    println!("Hello, {}!", name);
    format!("Hello, {}!", name)
}

async fn are_you_ready() -> impl IntoResponse {
    println!("I'm ready!");
    "I'm ready!"
}

async fn fallback_404() -> impl IntoResponse {
    tracing::info!("Got request, but no route was found.");
    (StatusCode::NOT_FOUND, "Route not found")
}
