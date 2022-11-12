use axum::{http::StatusCode, Extension, Json};
use entity::drug;
use entity::drug::NewDrug;
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel};

pub async fn get_drugs(Extension(ref db): Extension<DatabaseConnection>) -> StatusCode {
    StatusCode::OK
}

pub async fn define_drug(
    Extension(ref db): Extension<DatabaseConnection>,
    Json(drug): Json<NewDrug>,
) -> Result<Json<drug::Model>, (StatusCode, &'static str)> {
    let drug = drug.into_active_model().insert(db).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to insert new drug into database",
        )
    })?;
    Ok(Json(drug))
}
