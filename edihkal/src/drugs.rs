use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use edihkal_core::drugs::Drug;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_drugs() -> StatusCode {
    StatusCode::OK
}

pub async fn define_drug(
    Extension(db_pool): Extension<Arc<PgPool>>,
    Json(drug): Json<Drug>,
) -> StatusCode {
    match sqlx::query!(
        r#"
        INSERT INTO drugs (id, name)
        VALUES ($1, $2)
        "#,
        Uuid::new_v4(),
        drug.name
    )
    .execute(db_pool.as_ref())
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
