use crate::error::AppError;
use crate::models::feature_flags::{
    ActiveModel as ActiveFeatureFlag, Entity as FeatureFlags, Model as FeatureFlag,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait, Set};
use serde::Deserialize;
use tracing::info;

//# @INFO: Allow partial update on feature_flags
//# This will be allowed (PATCH) /api/v1/feature_flags/some_feature_flag
//# Payload: { enabled: false }
#[derive(Deserialize)]
pub struct PartialFeatureFlag {
    description: Option<String>,
    enabled: Option<bool>,
}

pub trait IntoFeatureFlag {
    fn into_feature_flag(self, existing: &FeatureFlag) -> FeatureFlag;
}

impl IntoFeatureFlag for PartialFeatureFlag {
    fn into_feature_flag(self, existing: &FeatureFlag) -> FeatureFlag {
        FeatureFlag {
            name: existing.name.clone(),
            description: self
                .description
                .unwrap_or_else(|| existing.description.clone()),
            enabled: self.enabled.unwrap_or(existing.enabled),
        }
    }
}

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

    pub async fn update(
        &self,
        id: &str,
        flag: PartialFeatureFlag,
    ) -> Result<FeatureFlag, AppError> {
        let existing_flag = self.get(id).await?;
        info!("Existing flag: {:?}", existing_flag);
        let updated_flag = flag.into_feature_flag(&existing_flag);
        info!("Updated flag: {:?}", updated_flag);

        let mut active_flag: ActiveFeatureFlag = existing_flag.clone().into();

        if updated_flag.description != existing_flag.description {
            active_flag.description = Set(updated_flag.description);
        }

        if updated_flag.enabled != existing_flag.enabled {
            active_flag.enabled = Set(updated_flag.enabled);
        }

        if active_flag.is_changed() {
            active_flag
                .update(&self.db)
                .await
                .map_err(AppError::DatabaseError)
        } else {
            Ok(existing_flag)
        }
    }

    pub async fn delete(&self, name: &str) -> Result<(), AppError> {
        let flag = self.get(name).await?;
        let _ = flag.delete(&self.db).await.map_err(AppError::DatabaseError);

        Ok(())
    }
}
