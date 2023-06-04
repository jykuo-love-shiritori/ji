use axum::{extract::State, http::StatusCode, Json};
use sea_query::{Alias, Expr, Query, SqliteQueryBuilder};
use sea_query_binder::SqlxBinder;

use crate::{AppState, Dead};

#[derive(sqlx::FromRow, Debug)]
struct DataStruct {
    year: i32,
    sum: i32,
}

#[derive(serde::Serialize)]
pub struct Response {
    year: Vec<i32>,
    total: Vec<i32>,
}

pub async fn dead_total_by_year(
    state: State<AppState>,
) -> Result<Json<Response>, (StatusCode, &'static str)> {
    let (sql, values) = Query::select()
        .column(Dead::Year)
        .expr_as(Expr::col(Dead::N).sum(), Alias::new("sum"))
        .from(Dead::Table)
        .group_by_col(Dead::Year)
        .build_sqlx(SqliteQueryBuilder);

    let results = sqlx::query_as_with::<_, DataStruct, _>(&sql, values.clone())
        .fetch_all(&state.pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong"))?;

    let (year, total): (Vec<i32>, Vec<i32>) = results
        .into_iter()
        .map(|result| (result.year, result.sum))
        .unzip();

    Ok(Json(Response { year, total }))
}
