use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheck<T> {
    status: String,
    available_routes: Vec<T>,
}

#[derive(Serialize)]
pub struct Route {
    name: String,
    method: String,
    path: String,
}

pub async fn status() -> (StatusCode, Json<HealthCheck<Route>>) {
    let available_routes = vec![
        Route {
            name: "[All Record]".to_string(),
            method: "GET".to_string(),
            path: "/api/v1/feature_flags".to_string(),
        },
        Route {
            name: "[Add Record]".to_string(),
            method: "POST".to_string(),
            path: "/api/v1/feature_flags".to_string(),
        },
        Route {
            name: "[Find Record]".to_string(),
            method: "GET".to_string(),
            path: "/api/v1/feature_flags/:name".to_string(),
        },
        Route {
            name: "[Update Record]".to_string(),
            method: "PUT".to_string(),
            path: "/api/v1/feature_flags/:name".to_string(),
        },
        Route {
            name: "[Delete Record]".to_string(),
            method: "DELETE".to_string(),
            path: "/api/v1/feature_flags/:name".to_string(),
        },
    ];

    let health_check = HealthCheck {
        status: "Ok".to_string(),
        available_routes,
    };

    (StatusCode::OK, Json(health_check))
}
