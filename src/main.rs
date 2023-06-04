use std::{env, net::SocketAddr, str::FromStr};

use axum::{
    http::Method,
    routing::{get, get_service},
    Router,
};
use sea_query::Iden;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
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

enum Dead {
    Table,
    Year,
    Cause,
    Sex,
    AgeCode,
    N,
}

impl Iden for Dead {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "dead",
                Self::Year => "year",
                Self::Cause => "cause",
                Self::Sex => "sex",
                Self::AgeCode => "age_code",
                Self::N => "N",
            }
        )
        .unwrap();
    }
}

#[derive(Clone)]
pub struct AppState {
    pool: SqlitePool,
}

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

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL")
        .expect("Environment variable `DATABASE_URL` not set or wrong `.env` file configuration");

    let opts = SqliteConnectOptions::from_str(&db_url)
        .expect("Invalid DB filename")
        .create_if_missing(true)
        .read_only(true);

    let pool = SqlitePool::connect_with(opts)
        .await
        .expect("Failed to create connection");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let state = AppState { pool };

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/hello", get(handler::hello_json))
                .route("/dead", get(handler::dead_test))
                .route("/dead_total_by_year", get(handler::dead_total_by_year))
                .route("/dead_total_by_cause", get(handler::dead_total_by_cause))
                .route(
                    "/dead_total_by_age_code",
                    get(handler::dead_total_by_age_code),
                ),
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
}
