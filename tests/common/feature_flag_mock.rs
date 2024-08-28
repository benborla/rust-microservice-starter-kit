use crate::common::create_mock_db;

use axum::Router;
use rust_microservice_starter_kit::{
    api::routes, models::feature_flags, services::feature_flag_service::FeatureFlagService,
};
use std::sync::Arc;

//# @INFO: Preloaded Feature Flag Mocked Data
pub fn create_feature_flag_service() -> FeatureFlagService {
    let mock_data = vec![
        feature_flags::Model {
            name: "new_flag".to_string(),
            description: "New test flag".to_string(),
            enabled: true,
        },
        feature_flags::Model {
            name: "flag1".to_string(),
            description: "Test flag 1".to_string(),
            enabled: true,
        },
        feature_flags::Model {
            name: "flag2".to_string(),
            description: "Test flag 2".to_string(),
            enabled: false,
        },
    ];

    let db = create_mock_db::<feature_flags::Entity>(mock_data);
    FeatureFlagService::new(db)
}

pub fn create_test_app(service: FeatureFlagService) -> Router {
    routes::create_router(Arc::new(service))
}
