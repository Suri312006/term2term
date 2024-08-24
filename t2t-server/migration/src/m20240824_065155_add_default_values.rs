use sea_orm::sqlx::encode::IsNull;
use sea_orm_migration::{prelude::*, schema::*};

use crate::m20240824_010034_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(User::Table)
                    .modify_column(
                        ColumnDef::new(User::CreatedAt).default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        //TODO: not sure why this dont work lmao
        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(User::Table)
                    .modify_column(ColumnDef::new(User::CreatedAt))
                    .to_owned(),
            )
            .await
    }
}
