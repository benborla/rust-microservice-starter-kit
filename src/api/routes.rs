use axum::{routing::get, Router};

use std::sync::Arc;

use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::api::handlers::feature_flags;
use crate::api::handlers::health_check;
use crate::services::feature_flag_service::FeatureFlagService;

pub fn create_router(feature_flag_service: Arc<FeatureFlagService>) -> Router {
    Router::new()
        //# Route "/" => health_check handler
        .route("/", get(health_check::status))
        //# Route [GET, POST]: /api/v1/feature_flags => feature_flags handler
        .route(
            "/api/v1/feature_flags",
            get(feature_flags::all).post(feature_flags::create),
        )
        //# Route [GET, PUT, DELETE]: /api/v1/feature_flags/[:name] => feature_flags handler
        .route(
            "/api/v1/featrure_flags/:name",
            get(feature_flags::get)
                .put(feature_flags::update)
                .delete(feature_flags::delete),
        )
        .with_state(feature_flag_service)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}
