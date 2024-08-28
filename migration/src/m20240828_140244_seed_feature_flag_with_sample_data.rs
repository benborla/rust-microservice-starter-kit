use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(FeatureFlag::Table)
            .columns([
                FeatureFlag::Name,
                FeatureFlag::Description,
                FeatureFlag::Enabled,
            ])
            .values_panic([
                Expr::value("allow_site_registration"),
                Expr::value("A basic description for allowing web registration"),
                Expr::value(true),
            ])
            .values_panic([
                Expr::value("allow_user_analytics"),
                Expr::value("Should the user be notified about analytics"),
                Expr::value(false),
            ])
            .values_panic([
                Expr::value("enable_loadbalancer"),
                Expr::value("Sample flag to check if loadbalancer is enabled"),
                Expr::value(true),
            ])
            .to_owned();

        manager.exec_stmt(insert).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let truncate = sea_query::Table::truncate()
            .table(FeatureFlag::Table)
            .to_owned();

        manager.truncate_table(truncate).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum FeatureFlag {
    Table,
    Name,
    Description,
    Enabled,
}
