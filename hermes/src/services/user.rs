use log::error;
use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

use crate::{
    grpc::{
        user_service_server::UserService, CreateUserReq, CreateUserRes, DeleteUserReq,
        DeleteUserRes, SearchUserReq, SearchUserRes, UpdateUserReq, UpdateUserRes, VerifyUserReq,
    },
    utils::Id,
};

//#[derive(Default)]
pub struct UserServer {
    pub db: Pool<Postgres>,
}

#[tonic::async_trait]
impl UserService for UserServer {
    async fn create(
        &self,
        request: Request<CreateUserReq>,
    ) -> Result<Response<CreateUserRes>, Status> {
        let request = request.into_inner();
        let x = sqlx::query!(
            r#"
        INSERT INTO Users (UserPubId, Name, Suffix)
        VALUES ($1, $2, $3)
        RETURNING *;
        "#,
            Id::gen(),
            request.name,
            request.suffix
        )
        .fetch_all(&self.db)
        .await
        .map_err(|err| {
            error!("{err}");
            Status::internal("xd")
        })?;

        for i in x {
            println!("{}", i.userpubid)
        }

        todo!()

        //Ok(Response::new(CreateUserRes {
        //    created_user: x,
        //    status: 0,
        //}))
    }

    async fn search(
        &self,
        request: Request<SearchUserReq>,
    ) -> Result<Response<SearchUserRes>, Status> {
        todo!()
    }

    async fn update(
        &self,
        request: Request<UpdateUserReq>,
    ) -> Result<Response<UpdateUserRes>, Status> {
        todo!()
    }

    async fn delete(
        &self,
        request: Request<DeleteUserReq>,
    ) -> Result<Response<DeleteUserRes>, Status> {
        todo!()
    }
    async fn verify(&self, request: Request<VerifyUserReq>) -> Result<Response<bool>, Status> {
        todo!()
    }
}

//#[cfg(test)]
//mod tests {
//    use color_eyre::eyre::Result;
//    use sea_orm::{prelude::*, Database, DatabaseBackend, MockDatabase, MockExecResult};
//    use tonic::IntoRequest;
//
//    use crate::{
//        entities::user,
//        services::{user_service_server::UserService, NewUserReq, UserServer},
//    };
//
//    #[tokio::test]
//    async fn create_user() -> Result<()> {
//        let db = MockDatabase::new(DatabaseBackend::Postgres)
//            .append_query_results(vec![[user::Model {
//                name: "Surendra".to_owned(),
//                pub_id: "123456789012345678901".to_owned(),
//                id: 1,
//                ..Default::default()
//            }]])
//            .append_exec_results(vec![MockExecResult {
//                last_insert_id: 1,
//                rows_affected: 1,
//            }])
//            .into_connection();
//
//        let server = UserServer { db };
//
//        let res = server
//            .create(
//                NewUserReq {
//                    name: "Surendra".to_owned(),
//                }
//                .into_request(),
//            )
//            .await?
//            .into_inner();
//
//        let transaction_lg = server.db.into_transaction_log();
//        println!("{transaction_lg:#?}");
//        assert_eq!(res.name.as_str(), "Surendra");
//        assert_eq!(res.id.as_str().len(), 21);
//        assert!(!transaction_lg.is_empty());
//        Ok(())
//    }
//}
