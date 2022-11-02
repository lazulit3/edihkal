use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use edihkal_core::drugs::{Drug, DrugInputs};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_drugs() -> StatusCode {
    StatusCode::OK
}

/// Define a new drug
#[tracing::instrument(name = "Defining new drug", skip(db_pool), fields(request_id = %Uuid::new_v4(), drug = drug.name))]
pub async fn define_drug(
    Extension(db_pool): Extension<Arc<PgPool>>,
    Json(drug): Json<DrugInputs>,
) -> Result<Json<Drug>, StatusCode> {
    match insert_drug(&db_pool, &drug).await {
        Ok(drug) => Ok(Json(drug)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Insert drug into database
#[tracing::instrument(name = "Saving new drug in database", skip_all)]
pub async fn insert_drug(db_pool: &PgPool, drug: &DrugInputs) -> Result<Drug, sqlx::Error> {
    match sqlx::query_as!(
        Drug,
        r#"
        INSERT INTO drugs (id, name)
        VALUES ($1, $2)
        RETURNING *
        "#,
        Uuid::new_v4(),
        drug.name
    )
    .fetch_one(db_pool)
    .await
    {
        Ok(drug) => Ok(drug),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            Err(e)
        }
    }
}
