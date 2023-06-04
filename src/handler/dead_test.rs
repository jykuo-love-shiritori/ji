use axum::{extract::State, http::StatusCode, Json};
use sea_query::{Query, SqliteQueryBuilder};
use sea_query_binder::SqlxBinder;

use crate::{AppState, Dead};

#[derive(sqlx::FromRow)]
struct YearStruct {
    year: i32,
}

pub async fn dead_test(
    state: State<AppState>,
) -> Result<Json<Vec<i32>>, (StatusCode, &'static str)> {
    let (sql, values) = Query::select()
        .column(Dead::Year)
        .from(Dead::Table)
        .build_sqlx(SqliteQueryBuilder);

    let results = sqlx::query_as_with::<_, YearStruct, _>(&sql, values.clone())
        .fetch_all(&state.pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong"))?;

    let names = results.into_iter().map(|result| result.year).collect();

    Ok(Json(names))
}
