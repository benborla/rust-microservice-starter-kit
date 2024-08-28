mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use rust_microservice_starter_kit::models::feature_flags::Model as FeatureFlag;
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot` and `ready`

use crate::common::feature_flag_mock;
use crate::common::read_body;

#[tokio::test]
async fn test_health_check() {
    let service = feature_flag_mock::create_feature_flag_service();
    let app = feature_flag_mock::create_test_app(service);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = read_body(response.into_body()).await.unwrap();
    let json: Value = serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Check the structure of the JSON response
    assert_eq!(json["status"], "Ok");
    assert!(json["available_routes"].is_array());
}

#[tokio::test]
async fn test_get_all_feature_flags() {
    let service = feature_flag_mock::create_feature_flag_service();
    let app = feature_flag_mock::create_test_app(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/feature_flags")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = read_body(response.into_body()).await.unwrap();
    let flags: Vec<FeatureFlag> = serde_json::from_slice(&body).unwrap();

    assert_eq!(flags.len(), 3);
    assert_eq!(flags[1].name, "flag1");
    assert_eq!(flags[2].name, "flag2");
}

#[tokio::test]
async fn test_delete_feature_flag() {
    let service = feature_flag_mock::create_feature_flag_service();
    let app = feature_flag_mock::create_test_app(service);

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/feature_flags/flag--sample-deleted")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::NO_CONTENT || response.status() == StatusCode::NOT_FOUND
    );
}

#[tokio::test]
async fn test_get_specific_feature_flag() {
    let service = feature_flag_mock::create_feature_flag_service();
    let app = feature_flag_mock::create_test_app(service);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/feature_flags/new_flag")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = read_body(response.into_body()).await.unwrap();
    let flag: FeatureFlag = serde_json::from_slice(&body).unwrap();

    assert_eq!(flag.name, "new_flag");
    assert_eq!(flag.description, "New test flag");
    assert_eq!(flag.enabled as i32, 1);
}

#[tokio::test]
async fn test_create_feature_flag() {
    let service = feature_flag_mock::create_feature_flag_service();
    let app = feature_flag_mock::create_test_app(service);

    let new_flag = json!({
        "name": "new_flag",
        "description": "New test flag",
        "enabled": true
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/feature_flags")
                .header("Content-Type", "application/json")
                .body(Body::from(new_flag.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = read_body(response.into_body()).await.unwrap();
    let created_flag: FeatureFlag = serde_json::from_slice(&body).unwrap();

    assert_eq!(created_flag.name, "new_flag");
    assert_eq!(created_flag.description, "New test flag");
    assert_eq!(created_flag.enabled as u8, 1);
}
