use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

use crate::grpc::{
    convo_service_server::ConvoService, CreateConvoReq, CreateConvoRes, DeleteConvoReq,
    DeleteConvoRes, SearchConvoReq, SearchConvoRes,
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
        &self,
        req: Request<SearchConvoReq>,
    ) -> Result<Response<SearchConvoRes>, Status> {
        todo!()
    }
    async fn create(
        &self,
        req: Request<CreateConvoReq>,
    ) -> Result<Response<CreateConvoRes>, Status> {
        todo!()
    }

    async fn delete(
        &self,
        req: Request<DeleteConvoReq>,
    ) -> Result<Response<DeleteConvoRes>, Status> {
        todo!()
    }
}
