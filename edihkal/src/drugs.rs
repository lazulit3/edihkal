use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use edihkal_core::drugs::{Drug, DrugInputs};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_drugs() -> StatusCode {
    StatusCode::OK
}

pub async fn define_drug(
    Extension(db_pool): Extension<Arc<PgPool>>,
) -> StatusCode {
    match sqlx::query!(
    Json(drug): Json<DrugInputs>,
    // Construct Drug with random id from payload.
    let drug = Drug {
        id: Uuid::new_v4(),
        name: drug.name,
    };
        r#"
        INSERT INTO drugs (id, name)
        VALUES ($1, $2)
        "#,
        drug.id,
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
