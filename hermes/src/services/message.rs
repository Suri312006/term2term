use std::sync::Arc;

use dashmap::DashMap;
use log::error;
use sqlx::{Pool, Postgres};
use tokio::sync::mpsc::{self, Sender};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::grpc::{
    msg_service_server::MsgService, DeleteMsgReq, DeleteMsgRes, Msg, RecieveRequest, SearchMsgReq,
    SearchMsgRes, SendMsgReq, SendMsgRes,
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
        tokio::spawn(async move {
            let req = req.into_inner();
            for recipient in req.convo.unwrap().participants.unwrap().inner {
                // its fine for us if the recipient isnt found inside of connections
                if let Some(conn) = self.connections.get(&recipient.id) {
                    conn.send(Ok(Msg {
                        id: "lol".to_string(),
                        author: None,
                        body: "wtf".to_string(),
                        convo: None,
                        is_read: false,
                        recipient: None,
                        unix_time: 123456,
                    }))
                    .await
                    .unwrap();
                };
            }
        });

        Ok(Response::new(SendMsgRes { sent_msg: None }))
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
