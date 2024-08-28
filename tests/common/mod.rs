pub mod feature_flag_mock;

use axum::body::to_bytes;
use axum::body::Body;
use bytes::Bytes;
use sea_orm::{DatabaseConnection, EntityTrait, MockDatabase, MockExecResult};

const MAX_BODY_SIZE: usize = 1024 * 1024; // 1 MB limit, adjust as needed
pub async fn read_body(body: Body) -> Result<Bytes, Box<dyn std::error::Error>> {
    Ok(to_bytes(body, MAX_BODY_SIZE).await?)
}

pub fn create_mock_db<E: EntityTrait>(mock_data: Vec<E::Model>) -> DatabaseConnection
where
    E::Model: Clone,
{
    MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
        .append_query_results([mock_data])
        .append_exec_results([MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .into_connection()
}
