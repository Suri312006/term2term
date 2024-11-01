use log::error;
use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

use crate::{
    entities::user::User,
    grpc::{
        self, user_service_server::UserService, CreateUserReq, CreateUserRes, DeleteUserReq,
        DeleteUserRes, Existence, SearchUserReq, SearchUserReqEnum, SearchUserRes, UpdateUserReq,
        UpdateUserRes, VerifyUserReq,
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

        // first check to make sure user name was already not taken
        if !sqlx::query!(
            r#"
            SELECT * 
            FROM Users 
            WHERE Name = $1 
            AND Suffix = $2"#,
            request.name,
            request.suffix,
        )
        .fetch_all(&self.db)
        .await
        .map_err(|err| {
            error!("{err}");
            Status::internal("xd")
        })?
        .is_empty()
        {
            return Ok(Response::new(CreateUserRes {
                status: Existence::AlreadyExists.into(),
                created_user: None,
            }));
        }

        // if not taken, then insert into it
        let x = sqlx::query_as!(
            User,
            r#"
        INSERT INTO Users (UserPubId, Name, Suffix)
        VALUES ($1, $2, $3)
        RETURNING UserPubId AS id, Name, Suffix;
        "#,
            Id::gen(),
            request.name,
            request.suffix
        )
        .fetch_one(&self.db)
        .await
        .map_err(|err| {
            error!("{err}");
            Status::internal("xd")
        })?;

        Ok(Response::new(CreateUserRes {
            created_user: Some(x.into()),
            status: Existence::Success.into(),
        }))
    }

    async fn search(
        &self,
        request: Request<SearchUserReq>,
    ) -> Result<Response<SearchUserRes>, Status> {
        let request = request.into_inner();

        let x = match request.r#type() {
            SearchUserReqEnum::Name => sqlx::query_as!(
                User,
                r#"
                SELECT UserPubId as id, Name, Suffix 
                FROM Users
                WHERE $1 = Name
                "#,
                request.name
            )
            .fetch_all(&self.db)
            .await
            .map_err(|err| {
                error!("{:#?}", err);
                Status::internal("db error")
            })?,

            SearchUserReqEnum::NameAndSuffix => sqlx::query_as!(
                User,
                r#"
                SELECT UserPubId as id, Name, Suffix 
                FROM Users
                WHERE $1 = Name AND $2 = Suffix
                "#,
                request.name,
                request.suffix
            )
            .fetch_all(&self.db)
            .await
            .map_err(|err| {
                error!("{:#?}", err);
                Status::internal("db error")
            })?,
        };

        Ok(Response::new(SearchUserRes {
            results: x.into_iter().map(Into::<grpc::User>::into).collect(),
        }))
    }

    async fn update(
        &self,
        request: Request<UpdateUserReq>,
    ) -> Result<Response<UpdateUserRes>, Status> {
        let request = request.into_inner();

        //let x = sqlx::query_as!(
        //    User,
        //    r#"
        //UPDATE Users
        //SET Name = $4, Suffix = $5
        //WHERE UserPubId = $1 AND Name = $2 AND Suffix = $3
        //RETURNING UserPubId AS id, Name, Suffix;
        //"#,
        //
        //);
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
