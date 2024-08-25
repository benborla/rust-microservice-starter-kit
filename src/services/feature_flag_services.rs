use crate::error::AppError;
use crate::models::feature_flag::FeatureFlag;

use sqlx::PgPool;

pub struct FeatureFlagService {
    pool: PgPool,
}

impl FeatureFlagService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<FeatureFlag>, Box<dyn std::error::Error>> {
        sqlx::query_as::<_, FeatureFlag>("SELECT name, description, enabled FROM feature_flags")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    pub async fn get(&self, name: &str) -> Result<FeatureFlag, AppError> {
        sqlx::query_as::<_, FeatureFlag>(
            "SELECT name, description, enabled FROM feature_flags WHERE name = $1",
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn create(&self, flag: FeatureFlag) -> Result<FeatureFlag, AppError> {
        sqlx::query_as::<_, FeatureFlag>(
            "INSERT INTO feature_flags (name, description, enabled) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(flag.name)
        .bind(flag.description)
        .bind(flag.enabled)
        .fetch_one(&self.pool)
        .await.
        map_err(AppError::DatabaseError)
    }

    pub async fn update(&self, flag: FeatureFlag) -> Result<FeatureFlag, AppError> {
        sqlx::query_as::<_, FeatureFlag>(
            "UPDATE feature_flags SET description = $2, enabled = $3 WHERE name = $1 RETURNING *",
        )
        .bind(&flag.name)
        .bind(flag.description)
        .bind(flag.enabled)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                AppError::NotFound(format!("Feature flag '{}' not found", flag.name))
            }
            _ => AppError::DatabaseError(e),
        })
    }

    pub async fn delete(&self, name: &str) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM feature_flags WHERE name = $1")
            .bind(name)
            .execute(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;

        if result.rows_affected() == 0 {
            Err(AppError::NotFound(format!(
                "Feature flag '{}' not found",
                name
            )))
        } else {
            Ok(())
        }
    }
}
