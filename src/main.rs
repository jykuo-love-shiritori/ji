use std::{env, net::SocketAddr};

use axum::{
    http::Method,
    routing::{get, get_service},
    Router,
};
use sea_query::{ColumnDef, Iden, Query, SqliteQueryBuilder, Table};
use sea_query_binder::SqlxBinder;
use sqlx::SqlitePool;
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

    let sql = Table::create()
        .table(Dead::Table)
        .if_not_exists()
        .col(ColumnDef::new(Dead::Name).string())
        .build(SqliteQueryBuilder);

    sqlx::query(&sql).execute(&pool).await?;

    let (sql, values) = Query::insert()
        .into_table(Dead::Table)
        .columns([Dead::Name])
        .values_panic(["Bob".into()])
        .values_panic(["Alex".into()])
        .values_panic(["Rick".into()])
        .build_sqlx(SqliteQueryBuilder);

    sqlx::query_with(&sql, values).execute(&pool).await?;

    let (sql, values) = Query::select()
        .columns([Dead::Name])
        .from(Dead::Table)
        .build_sqlx(SqliteQueryBuilder);

    let results = sqlx::query_as_with::<_, DeadStruct, _>(&sql, values.clone())
        .fetch_all(&pool)
        .await?;

    for result in results.iter() {
        println!("{:?}", result);
    }

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .route("/hello", get(handler::hello_json))
        .nest_service(
            "/",
            get_service(ServeDir::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/frontend/dist"
            ))),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    println!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to start server");

    let sql = Table::drop()
        .table(Dead::Table)
        .if_exists()
        .build(SqliteQueryBuilder);

    sqlx::query(&sql).execute(&pool).await?;

    Ok(())
}
