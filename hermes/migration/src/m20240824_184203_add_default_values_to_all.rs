use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20240824_010034_create_user_table::User,
    m20240824_010041_create_conversation_table::Conversation,
    m20240824_010043_create_message_table::Message,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(User::Table)
                    .add_column(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(Conversation::Table)
                    .add_column(
                        ColumnDef::new(Conversation::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(Message::Table)
                    .add_column(
                        ColumnDef::new(Message::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(User::Table)
                    .drop_column(User::CreatedAt)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(Conversation::Table)
                    .drop_column(Conversation::CreatedAt)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(Message::Table)
                    .drop_column(Message::CreatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
