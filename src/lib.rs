pub mod api;
pub mod config;
pub mod db;
pub mod error;
pub mod models;
pub mod services;

use axum::Router;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::api::routes::create_router;
use crate::error::AppError;
use crate::services::feature_flag_service::FeatureFlagService;

pub async fn run_app() -> Result<Router, AppError> {
    let connection = db::connect().await?;

    let feature_flag_service = Arc::new(FeatureFlagService::new(connection));
    let app = create_router(feature_flag_service).layer(TraceLayer::new_for_http());

    Ok(app)
}
