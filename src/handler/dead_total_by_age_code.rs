use axum::{extract::State, http::StatusCode, Json};
use sea_query::{Alias, Expr, Order, Query, SqliteQueryBuilder};
use sea_query_binder::SqlxBinder;

use crate::{AppState, Dead};

#[derive(sqlx::FromRow, Debug)]
struct DataStruct {
    sex: i32,
    sum: i32,
}

#[derive(serde::Serialize)]
pub struct Response {
    age_code: Vec<String>,
    total_male: Vec<i32>,
    total_female: Vec<i32>,
}

pub async fn dead_total_by_age_code(
    state: State<AppState>,
) -> Result<Json<Response>, (StatusCode, &'static str)> {
    let (sql, values) = Query::select()
        .column(Dead::Sex)
        .expr_as(Expr::col(Dead::N).sum(), Alias::new("sum"))
        .from(Dead::Table)
        .group_by_columns([Dead::Sex, Dead::AgeCode])
        .order_by(Dead::AgeCode, Order::Desc)
        .build_sqlx(SqliteQueryBuilder);

    let results = sqlx::query_as_with::<_, DataStruct, _>(&sql, values.clone())
        .fetch_all(&state.pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong"))?;

    let age_code = vec![
        "不詳",
        "100歲以上",
        "95-99歲",
        "90-94歲",
        "85-89歲",
        "80-84歲",
        "75-79歲",
        "70-74歲",
        "65-69歲",
        "60-64歲",
        "55-59歲",
        "50-54歲",
        "45-49歲",
        "40-44歲",
        "35-39歲",
        "30-34歲",
        "25-29歲",
        "20-24歲",
        "15-19歲",
        "10-14歲",
        "5-9歲",
        "四歲",
        "三歲",
        "二歲",
        "一歲",
        "嬰兒(滿四週至未滿一歲)",
        "新生兒﹝未滿四週﹞",
    ]
    .into_iter()
    .map(String::from)
    .collect();

    let total_male = results
        .iter()
        .filter(|result| result.sex == 1)
        .map(|result| result.sum)
        .collect();

    let total_female = results
        .iter()
        .filter(|result| result.sex == 2)
        .map(|result| result.sum)
        .collect();

    Ok(Json(Response {
        age_code,
        total_male,
        total_female,
    }))
}
