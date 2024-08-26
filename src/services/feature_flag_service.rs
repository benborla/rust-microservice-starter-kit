use crate::error::AppError;
use crate::models::feature_flags::{
    ActiveModel as ActiveFeatureFlag, Entity as FeatureFlags, Model as FeatureFlag,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait, Set};

pub struct FeatureFlagService {
    db: DatabaseConnection,
}

impl FeatureFlagService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_all(&self) -> Result<Vec<FeatureFlag>, AppError> {
        FeatureFlags::find()
            .all(&self.db)
            .await
            .map_err(AppError::DatabaseError)
    }

    pub async fn create(&self, flag: FeatureFlag) -> Result<FeatureFlag, AppError> {
        let active_flag = ActiveFeatureFlag {
            name: Set(flag.name),
            description: Set(flag.description),
            enabled: Set(flag.enabled),
        };

        active_flag
            .insert(&self.db)
            .await
            .map_err(AppError::DatabaseError)
    }

    pub async fn get(&self, name: &str) -> Result<FeatureFlag, AppError> {
        FeatureFlags::find_by_id(name)
            .one(&self.db)
            .await
            .map_err(AppError::DatabaseError)?
            .ok_or_else(|| AppError::NotFound(format!("Feature flag '{}' not found", name)))
    }

    pub async fn update(&self, flag: FeatureFlag) -> Result<FeatureFlag, AppError> {
        let existing_flag = self.get(&flag.name).await?;

        let mut active_flag: ActiveFeatureFlag = existing_flag.into();
        active_flag.description = Set(flag.description);
        active_flag.enabled = Set(flag.enabled);

        active_flag
            .update(&self.db)
            .await
            .map_err(AppError::DatabaseError)
    }

    pub async fn delete(&self, name: &str) -> Result<(), AppError> {
        let flag = self.get(name).await?;
        let _ = flag.delete(&self.db).await.map_err(AppError::DatabaseError);

        Ok(())
    }
}
