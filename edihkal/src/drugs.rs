use aide::transform::TransformOperation;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use schemars::JsonSchema;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, JsonSchema)]
pub struct Drug {
    name: String,
}

pub async fn get_drugs() -> StatusCode {
    StatusCode::OK
}

pub async fn define_drug(State(db_pool): State<PgPool>, Json(drug): Json<Drug>) -> StatusCode {
    match sqlx::query!(
        r#"
        INSERT INTO drugs (id, name)
        VALUES ($1, $2)
        "#,
        Uuid::new_v4(),
        drug.name
    )
    .execute(&db_pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub fn define_drug_docs(op: TransformOperation) -> TransformOperation {
    op.description("Define a new drug.")
        .response_with::<200, (), _>(|res| res.description("Drug has been defined."))
        .response_with::<500, (), _>(|res| res.description("Failed to define drug."))
}
