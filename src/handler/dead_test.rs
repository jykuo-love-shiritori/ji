use axum::{extract::State, http::StatusCode, Json};
use sea_query::{Query, SqliteQueryBuilder};
use sea_query_binder::SqlxBinder;

use crate::{AppState, Dead, DeadStruct};

pub async fn dead_test(
    state: State<AppState>,
) -> Result<Json<Vec<String>>, (StatusCode, &'static str)> {
    let (sql, values) = Query::select()
        .columns([Dead::Name])
        .from(Dead::Table)
        .build_sqlx(SqliteQueryBuilder);

    let results = sqlx::query_as_with::<_, DeadStruct, _>(&sql, values.clone())
        .fetch_all(&state.pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong"))?;

    let names = results.into_iter().map(|result| result.name).collect();

    Ok(Json(names))
}
