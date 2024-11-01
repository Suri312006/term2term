use log::error;
use sqlx::{Pool, Postgres};
use tonic::{async_trait, Request, Response, Status};

use crate::{
    grpc::{
        auth_service_server::AuthService, CreateUserRes, Existence, LoginReq, RegisterReq,
        RegisterRes, Token, User,
    },
    utils::Id,
};

//https://dev.to/martinp/roll-your-own-auth-with-rust-and-protobuf-24ke
pub struct AuthServer {
    db: Pool<Postgres>,
}

#[async_trait]
impl AuthService for AuthServer {
    async fn login(&self, req: Request<LoginReq>) -> Result<Response<Token>, Status> {
        todo!()
    }
    async fn register(&self, req: Request<RegisterReq>) -> Result<Response<RegisterRes>, Status> {
        todo!()
        //let req = req.into_inner();
        //
        //let password = bcrypt::hash(&req.password, 10)
        //    .map_err(|_| Status::unknown("Error while creating user"))?;
        //
        //if !sqlx::query!(
        //    r#"
        //    SELECT *
        //    FROM Users
        //    WHERE Name = $1
        //    AND Suffix = $2"#,
        //    req.name,
        //    req.suffix,
        //)
        //.fetch_all(&self.db)
        //.await
        //.map_err(|err| {
        //    error!("{err}");
        //    Status::internal("xd")
        //})?
        //.is_empty()
        //{
        //    return Ok(Response::new(CreateUserRes {
        //        status: Existence::AlreadyExists.into(),
        //        created_user: None,
        //    }));
        //}
        //
        //// if not taken, then insert into it
        //let x = sqlx::query_as!(
        //    User,
        //    r#"
        //INSERT INTO Users (UserPubId, Name, Suffix)
        //VALUES ($1, $2, $3)
        //RETURNING UserPubId AS id, Name, Suffix;
        //"#,
        //    Id::gen(),
        //    req.name,
        //    req.suffix
        //)
        //.fetch_one(&self.db)
        //.await
        //.map_err(|err| {
        //    error!("{err}");
        //    Status::internal("xd")
        //})?;
        //
        //Ok(Response::new(RegisterRes {
        //    created_user: Some(x.into()),
        //    status: Existence::Success.into(),
        //}))
    }
    async fn verify_token(&self, req: Request<Token>) -> Result<Response<User>, Status> {
        todo!()
    }
}
