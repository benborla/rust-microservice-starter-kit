pub use sea_orm_migration::prelude::*;

mod m20240828_134140_create_feature_flag_table;
mod m20240828_140244_seed_feature_flag_with_sample_data;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240828_134140_create_feature_flag_table::Migration),
            Box::new(m20240828_140244_seed_feature_flag_with_sample_data::Migration),
        ]
    }
}
