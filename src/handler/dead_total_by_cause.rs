use axum::{extract::State, http::StatusCode, Json};
use sea_query::{Alias, Expr, Order, Query, SqliteQueryBuilder};
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
        .order_by(Alias::new("sum"), Order::Desc)
        .build_sqlx(SqliteQueryBuilder);

    let results = sqlx::query_as_with::<_, DataStruct, _>(&sql, values.clone())
        .fetch_all(&state.pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong"))?;

    let other_total = results
        .iter()
        .find(|&result| result.cause == "其他")
        .unwrap()
        .sum;

    let (cause, total): (Vec<String>, Vec<i32>) = results
        .into_iter()
        .filter_map(|result| {
            if result.cause != "其他" {
                Some((result.cause, result.sum))
            } else {
                None
            }
        })
        .unzip();

    let top = 7;

    let cause = cause[0..top]
        .iter()
        .cloned()
        .chain([String::from("其他")])
        .collect();

    let other_total = total[top..].iter().sum::<i32>() + other_total;

    let total = total[0..top].iter().cloned().chain([other_total]).collect();

    Ok(Json(Response { cause, total }))
}
