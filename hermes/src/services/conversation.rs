use std::sync::Arc;

use log::error;
use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

use crate::{
    entities::{self, Conversation},
    grpc::{
        convo_service_server::ConvoService, CreateConvoReq, CreateConvoRes, DeleteConvoReq,
        DeleteConvoRes, Existence, SearchConvoReq, SearchConvoRes,
    },
    utils::Id,
};

pub struct ConversationServer {
    db: Pool<Postgres>,
}

impl ConversationServer {
    pub fn new(db: Pool<Postgres>) -> Self {
        ConversationServer { db }
    }
}

#[tonic::async_trait]
impl ConvoService for ConversationServer {
    async fn search(
        self: Arc<Self>,
        req: Request<SearchConvoReq>,
    ) -> Result<Response<SearchConvoRes>, Status> {
        todo!()
    }
    async fn create(
        self: Arc<Self>,
        req: Request<CreateConvoReq>,
    ) -> Result<Response<CreateConvoRes>, Status> {
        let req = req.into_inner();
        let convoids = sqlx::query!(
            r#"
        INSERT INTO Conversations (ConvoPubId)
        VALUES ($1)
        RETURNING ConvoId, ConvoPubId
        "#,
            Id::gen()
        )
        .fetch_one(&self.db)
        .await
        .map_err(|err| {
            error!("{err}");
            Status::internal("Failed to create convo")
        })?;

        for user in req
            .participants
            .clone()
            .ok_or_else(|| Status::invalid_argument("Expected participants"))?
            .inner
        {
            let inserted_link = sqlx::query!(
                r#"
                WITH UserSelection AS (
                    SELECT UserId
                    FROM Users
                    WHERE UserPubId = $1
                )
                INSERT INTO Users_Conversations (UserId, ConvoId)
                SELECT UserId, $2
                FROM UserSelection
                "#,
                user.id,
                convoids.convoid
            )
            .execute(&self.db)
            .await
            .map_err(|err| {
                error!("{err}");
                Status::internal("Failed to create convo")
            })?;

            assert!(inserted_link.rows_affected() > 0);
        }

        Ok(Response::new(CreateConvoRes {
            status: Existence::Success.into(),
            created: Some(
                Conversation {
                    id: convoids.convopubid,
                    participants: req
                        .participants
                        .unwrap()
                        .inner
                        .into_iter()
                        .map(Into::<entities::User>::into)
                        .collect(),
                }
                .into(),
            ),
        }))
    }

    async fn delete(
        self: Arc<Self>,
        req: Request<DeleteConvoReq>,
    ) -> Result<Response<DeleteConvoRes>, Status> {
        todo!()
    }
}
