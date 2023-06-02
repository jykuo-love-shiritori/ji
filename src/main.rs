use std::{env, net::SocketAddr};

use axum::{
    http::Method,
    routing::{get, get_service},
    Router,
};
use sea_query::Iden;
use sqlx::SqlitePool;
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{
    filter::{self, LevelFilter},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

mod handler;
mod shutdown;

use shutdown::shutdown_signal;

#[derive(Iden)]
enum Dead {
    Table,
    Name,
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
struct DeadStruct {
    name: String,
}

#[derive(Clone)]
pub struct AppState {
    pool: SqlitePool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = filter::Targets::new()
        .with_target("tower_http::trace::on_response", LevelFilter::TRACE)
        .with_target("tower_http::trace::on_request", LevelFilter::TRACE)
        .with_target("tower_http::trace::make_span", LevelFilter::DEBUG)
        .with_default(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();

    let pool = SqlitePool::connect(&db_url).await?;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let state = AppState { pool };

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/hello", get(handler::hello_json))
                .route("/dead", get(handler::dead_test)),
        )
        .nest_service(
            "/assets",
            get_service(ServeDir::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/frontend/dist/assets"
            ))),
        )
        .nest_service(
            "/",
            get_service(ServeFile::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/frontend/dist/index.html"
            ))),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    println!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to start server");

    Ok(())
}
