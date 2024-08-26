use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_macros::debug_handler;

use std::sync::Arc;

use crate::error::AppError;
use crate::models::feature_flags::Model as FeatureFlag;
use crate::services::feature_flag_service::FeatureFlagService;

#[debug_handler]
pub async fn all(
    State(service): State<Arc<FeatureFlagService>>,
) -> Result<Json<Vec<FeatureFlag>>, (StatusCode, String)> {
    service.get_all().await.map(Json).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch feature flags: {}", err),
        )
    })
}

#[debug_handler]
pub async fn create(
    State(service): State<Arc<FeatureFlagService>>,
    Json(flag): Json<FeatureFlag>,
) -> Result<(StatusCode, Json<FeatureFlag>), AppError> {
    service
        .create(flag)
        .await
        .map(|flag| (StatusCode::CREATED, Json(flag)))
}

#[debug_handler]
pub async fn get(
    State(service): State<Arc<FeatureFlagService>>,
    Path(name): Path<String>,
) -> Result<Json<FeatureFlag>, AppError> {
    service.get(&name).await.map(Json)
}

#[debug_handler]
pub async fn update(
    State(service): State<Arc<FeatureFlagService>>,
    Path(name): Path<String>,
    Json(mut flag): Json<FeatureFlag>,
) -> Result<Json<FeatureFlag>, AppError> {
    flag.name = name;
    service.update(flag).await.map(Json)
}

#[debug_handler]
pub async fn delete(
    State(service): State<Arc<FeatureFlagService>>,
    Path(name): Path<String>,
) -> Result<StatusCode, AppError> {
    service.delete(&name).await.map(|_| StatusCode::NO_CONTENT)
}
