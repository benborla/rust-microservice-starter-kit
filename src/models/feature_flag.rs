use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FeatureFlag {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}
