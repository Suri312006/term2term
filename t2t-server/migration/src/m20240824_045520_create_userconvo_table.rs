use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20240824_010034_create_user_table::User,
    m20240824_010049_create_conversation_table::Conversation,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(UserConvo::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .table(UserConvo::Table)
                            .col(UserConvo::UserId)
                            .col(UserConvo::ConvoId),
                    )
                    .col(integer(UserConvo::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserConvo::Table, UserConvo::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(integer(UserConvo::ConvoId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserConvo::Table, UserConvo::ConvoId)
                            .to(Conversation::Table, Conversation::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserConvo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserConvo {
    Table,
    Id,
    UserId,
    ConvoId,
}
