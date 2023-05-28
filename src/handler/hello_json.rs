use axum::Json;

#[derive(serde::Serialize)]
pub struct Test {
    message: String,
}

pub async fn hello_json() -> Json<Test> {
    Json(Test {
        message: String::from("hello"),
    })
}
