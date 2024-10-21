use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20240824_010034_create_user_table::User,
    m20240824_010041_create_conversation_table::Conversation,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(pk_auto(Message::Id))
                    .col(string(Message::PubId))
                    .col(string(Message::Body))
                    .col(integer(Message::AuthorId))
                    .col(integer(Message::ConversationId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-msg-convo_id")
                            .from(Message::Table, Message::ConversationId)
                            .to(Conversation::Table, Conversation::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-msg-user_id")
                            .from(Message::Table, Message::AuthorId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Message {
    Table,
    Id,
    PubId,
    AuthorId,
    ConversationId,
    CreatedAt,
    Body,
}
