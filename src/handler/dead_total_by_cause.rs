use axum::{extract::State, http::StatusCode, Json};
use sea_query::{Alias, Expr, Query, SqliteQueryBuilder};
use sea_query_binder::SqlxBinder;

use crate::{AppState, Dead};

#[derive(sqlx::FromRow, Debug)]
struct DataStruct {
    cause: String,
    sum: i32,
}

#[derive(serde::Serialize)]
pub struct Response {
    cause: Vec<String>,
    total: Vec<i32>,
}

pub async fn dead_total_by_cause(
    state: State<AppState>,
) -> Result<Json<Response>, (StatusCode, &'static str)> {
    let (sql, values) = Query::select()
        .column(Dead::Cause)
        .expr_as(Expr::col(Dead::N).sum(), Alias::new("sum"))
        .from(Dead::Table)
        .group_by_col(Dead::Cause)
        .build_sqlx(SqliteQueryBuilder);

    let results = sqlx::query_as_with::<_, DataStruct, _>(&sql, values.clone())
        .fetch_all(&state.pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong"))?;

    let (cause, total): (Vec<String>, Vec<i32>) = results
        .into_iter()
        .map(|result| (result.cause, result.sum))
        .unzip();

    Ok(Json(Response { cause, total }))
}
