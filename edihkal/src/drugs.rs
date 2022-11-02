use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use edihkal_core::drugs::{Drug, DrugInputs};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_drugs() -> StatusCode {
    StatusCode::OK
}

#[tracing::instrument(name = "Defining a new drug", skip(db_pool), fields(request_id = %Uuid::new_v4(), drug = drug.name))]
pub async fn define_drug(
    Extension(db_pool): Extension<Arc<PgPool>>,
    Json(drug): Json<DrugInputs>,
) -> Result<Json<Drug>, StatusCode> {
    // Construct Drug with random id from payload.
    let drug = Drug {
        id: Uuid::new_v4(),
        name: drug.name,
    };

    match sqlx::query_as!(
        Drug,
        r#"
        INSERT INTO drugs (id, name)
        VALUES ($1, $2)
        RETURNING *
        "#,
        drug.id,
        drug.name
    )
    .fetch_one(db_pool.as_ref())
    .await
    {
        Ok(drug) => Ok(Json(drug)),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
