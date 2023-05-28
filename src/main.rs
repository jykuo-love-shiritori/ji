use std::net::SocketAddr;

use axum::{
    http::Method,
    routing::{get, get_service},
    Router,
};
use tokio::signal;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{
    filter::{self, LevelFilter},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

mod handler;

#[tokio::main]
async fn main() {
    let filter = filter::Targets::new()
        .with_target("tower_http::trace::on_response", LevelFilter::TRACE)
        .with_target("tower_http::trace::on_request", LevelFilter::TRACE)
        .with_target("tower_http::trace::make_span", LevelFilter::DEBUG)
        .with_default(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .route("/hello", get(handler::hello_json))
        .nest_service("/", get_service(ServeDir::new("./frontend/dist")))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    println!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to start server");
}

// https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
