use core::error;

use log::error;
use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

use crate::{
    entities::user::User,
    grpc::{
        self, user_service_server::UserService, DeleteUserReq, DeleteUserRes, Existence,
        SearchUserReq, SearchUserReqEnum, SearchUserRes, UpdateUserReq, UpdateUserRes,
    },
};
//#[derive(Default)]
pub struct UserServer {
    db: Pool<Postgres>,
}

impl UserServer {
    pub fn new(db: Pool<Postgres>) -> Self {
        UserServer { db }
    }
}

#[tonic::async_trait]
impl UserService for UserServer {
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

    async fn update(&self, req: Request<UpdateUserReq>) -> Result<Response<UpdateUserRes>, Status> {
        let (headers, ext, req) = req.into_parts();
        let user_id = headers
            .get("user_id")
            .ok_or({
                error!("User id was not passed down into headers!!");
                Status::internal("try again later")
            })?
            .to_str()
            .map_err(|err| {
                error!("{err}");
                Status::internal("weird.")
            })?;

        // checks if there is already some user with the requested name
        if !sqlx::query!(
            r#"
        SELECT *
        FROM Users
        WHERE Name = $1 AND Suffix = $2
        "#,
            req.new_name,
            req.new_suffix
        )
        .fetch_all(&self.db)
        .await
        .map_err(|err| {
            error!("{:#?}", err);
            Status::internal("db error")
        })?
        .is_empty()
        {
            return Ok(Response::new(UpdateUserRes {
                status: Existence::AlreadyExists.into(),
                updated_user: None,
            }));
        }

        // now we can insert the new one

        let new_user = sqlx::query_as!(
            User,
            r#"
        UPDATE Users
        SET Name = $1, Suffix = $2
        WHERE UserPubId = $3
        RETURNING UserPubId AS id, Name, Suffix;
        "#,
            req.new_name,
            req.new_suffix,
            user_id
        )
        .fetch_one(&self.db)
        .await
        .map_err(|err| {
            error!("{err}");
            Status::internal("failed to modify user")
        })?;

        Ok(Response::new(UpdateUserRes {
            status: Existence::Success.into(),
            updated_user: Some(new_user.into()),
        }))
    }

    async fn delete(&self, req: Request<DeleteUserReq>) -> Result<Response<DeleteUserRes>, Status> {
        let (headers, _ext, _req) = req.into_parts();
        let user_id = headers
            .get("user_id")
            .ok_or({
                error!("User id was not passed down into headers!!");
                Status::internal("try again later")
            })?
            .to_str()
            .map_err(|err| {
                error!("{err}");
                Status::internal("weird.")
            })?;

        if sqlx::query!(
            r#"
        DELETE FROM Users WHERE UserPubId = $1;
        "#,
            user_id,
        )
        .fetch_all(&self.db)
        .await
        .map_err(|err| {
            error!("{err}");
            Status::internal("failed to delete user")
        })?
        .len()
            != 1
        {
            //BAD BAD BAD BAD BAD HOW DOES THIS HAPPEN
            // assuming that its zero len so nothing got deleted
            let var_name = DeleteUserRes { success: false };
            return Ok(Response::new(var_name));
        }

        // means delete was successfull
        Ok(Response::new(DeleteUserRes { success: false }))
    }
}
