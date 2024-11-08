use std::sync::Arc;

use dashmap::DashMap;
use log::error;
use sqlx::{Pool, Postgres};
use tokio::sync::mpsc::{self, Sender};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::{
    entities::{Conversation, Message},
    grpc::{
        msg_service_server::MsgService, DeleteMsgReq, DeleteMsgRes, Msg, RecieveRequest,
        SearchMsgReq, SearchMsgRes, SendMsgReq, SendMsgRes,
    },
    utils::Id,
};

#[derive(Debug)]
pub struct MessageServer {
    db: Pool<Postgres>,
    connections: DashMap<String, Sender<Result<Msg, Status>>>,
}

impl MessageServer {
    pub fn new(db: Pool<Postgres>) -> Self {
        MessageServer {
            db,
            connections: DashMap::new(),
        }
    }
}
//NOTE: tutorial: https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
// more perf: https://github.com/fujita/tokio-reuseport/tree/master/greeter-reuseport/src

#[tonic::async_trait]
impl MsgService for MessageServer {
    type RecieveIncomingStream = ReceiverStream<Result<Msg, Status>>;

    async fn recieve_incoming(
        self: Arc<Self>,
        req: Request<RecieveRequest>,
    ) -> Result<Response<Self::RecieveIncomingStream>, Status> {
        let (headers, _ext, _req) = req.into_parts();
        let user_id = headers
            .get("user_id")
            .ok_or_else(|| {
                error!("User id was not passed down into headers!!");
                Status::internal("try again later")
            })?
            .to_str()
            .map_err(|err| {
                error!("{err}");
                Status::internal("weird.")
            })?;
        let (tx, rx) = mpsc::channel::<Result<Msg, Status>>(4);

        //NOTE: may deadlock?? so idk
        let _ = self.connections.insert(user_id.to_string(), tx);

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn send(
        self: Arc<Self>,
        req: Request<SendMsgReq>,
    ) -> Result<Response<SendMsgRes>, Status> {
        let (headers, ext, req) = req.into_parts();
        let user_id = headers
            .get("user_id")
            .ok_or_else(|| {
                error!("User id was not passed down into headers!!");
                Status::internal("try again later")
            })?
            .to_str()
            .map_err(|err| {
                error!("{err}");
                Status::internal("weird.")
            })?;

        let convo_id = req
            .convo
            .clone()
            .ok_or_else(|| Status::invalid_argument("Did not pass in Conversation into request"))?
            .id;

        let executor = &self.clone().db;

        let record = sqlx::query!(
            r#"
WITH UserSelection AS (
    SELECT UserId
    FROM Users
    WHERE UserPubId = $1
),
ConvoSelection AS (
    SELECT ConvoId
    FROM Conversations
    WHERE ConvoPubId = $2
),
InsertMessage AS (
    INSERT INTO Messages (MsgPubId, AuthorId, ParentConvoId, Body)
    VALUES (
        $3, 
        (SELECT UserId FROM UserSelection),
        (SELECT ConvoId FROM ConvoSelection),
        $4
    )
    RETURNING *
)
SELECT 
    InsertMessage.MsgPubId AS id,
    InsertMessage.Body,
    InsertMessage.CreatedAt AS MsgCreatedAt,
    Users.UserPubId,
    Users.Name,
    Users.Suffix,
    Conversations.ConvoPubId
FROM
    InsertMessage
JOIN 

    Users ON Users.UserId = InsertMessage.AuthorId
JOIN 
    Conversations ON Conversations.ConvoId = InsertMessage.ParentConvoId
    
"#,
            user_id,
            convo_id,
            Id::gen(),
            req.body
        )
        .fetch_one(executor)
        .await
        .map_err(|err| {
            error!("{err}");
            Status::internal("unknown")
        })?;

        let message = Msg {
            id: record.id,
            convo: Some(req.convo.clone().unwrap()),
            author: Some(crate::grpc::User {
                id: record.userpubid,
                name: record.name,
                suffix: record.suffix,
            }),
            body: record.body.ok_or_else(|| {
                Status::internal("Something weird happened while trying to create message")
            })?,
            is_read: false,
            unix_time: record.msgcreatedat.timestamp() as u64,
        };

        let ret_message = message.clone();

        tokio::spawn(async move {
            for recipient in req.convo.unwrap().participants.unwrap().inner {
                // its fine for us if the recipient isnt found inside of connections
                if let Some(conn) = self.clone().connections.get(&recipient.id) {
                    //TODO: this actually needs to be the message we wanna send
                    conn.send(Ok(message.clone())).await.unwrap();
                };
            }
        });

        Ok(Response::new(SendMsgRes {
            sent_msg: Some(ret_message),
        }))
    }

    async fn delete(
        self: Arc<Self>,
        req: Request<DeleteMsgReq>,
    ) -> Result<Response<DeleteMsgRes>, Status> {
        todo!()
    }

    async fn search(
        self: Arc<Self>,
        req: Request<SearchMsgReq>,
    ) -> Result<Response<SearchMsgRes>, Status> {
        todo!()
    }
}
