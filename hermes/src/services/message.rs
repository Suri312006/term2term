use std::sync::Arc;

use sqlx::{Pool, Postgres};
use tokio::sync::{
    mpsc::{self, Sender},
    Mutex,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::grpc::{
    msg_service_server::MsgService, DeleteMsgReq, DeleteMsgRes, Msg, RecieveRequest, SearchMsgReq,
    SearchMsgRes, SendMsgReq, SendMsgRes,
};

#[derive(Debug)]
pub struct MessageServer {
    db: Pool<Postgres>,
    senders: Mutex<Vec<Sender<Result<Msg, Status>>>>,
}

impl MessageServer {
    pub fn new(db: Pool<Postgres>) -> Self {
        MessageServer {
            db,
            senders: Mutex::new(vec![].into()),
        }
    }
}
//NOTE: tutorial: https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md
// more perf: https://github.com/fujita/tokio-reuseport/tree/master/greeter-reuseport/src

#[tonic::async_trait]
impl MsgService for MessageServer {
    type RecieveIncomingStream = ReceiverStream<Result<Msg, Status>>;

    async fn recieve_incoming(
        &self,
        req: Request<RecieveRequest>,
    ) -> Result<Response<Self::RecieveIncomingStream>, Status> {
        let (mut tx, rx) = mpsc::channel::<Result<Msg, Status>>(4);

        let x = self.senders.lock().await.push(tx);

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn send(&self, req: Request<SendMsgReq>) -> Result<Response<SendMsgRes>, Status> {
        let mut senders = self.senders.lock().await;
        let y = senders.get_mut(0).unwrap();

        y.send(Ok(Msg {
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

        Ok(Response::new(SendMsgRes { sent_msg: None }))
    }

    async fn delete(&self, req: Request<DeleteMsgReq>) -> Result<Response<DeleteMsgRes>, Status> {
        todo!()
    }

    async fn search(&self, req: Request<SearchMsgReq>) -> Result<Response<SearchMsgRes>, Status> {
        todo!()
    }
}
