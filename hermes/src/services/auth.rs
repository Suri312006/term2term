use bcrypt::{hash, verify, DEFAULT_COST};
use email_address::EmailAddress;
use log::error;
use sqlx::{Pool, Postgres};
use tonic::{async_trait, Code, Request, Response, Status};

use crate::{
    entities::user::User,
    grpc::{
        self, auth_service_server::AuthService, CreateUserRes, Existence, LoginReq, LoginRes,
        LoginResEnum, RegisterReq, RegisterRes, Token,
    },
    middleware::auth::Auth,
    utils::Id,
};

//https://dev.to/martinp/roll-your-own-auth-with-rust-and-protobuf-24ke
pub struct AuthServer {
    db: Pool<Postgres>,
    auth: Auth,
}

impl AuthServer {
    pub fn new(db: Pool<Postgres>, auth: Auth) -> Self {
        AuthServer { db, auth }
    }
}

#[async_trait]
impl AuthService for AuthServer {
    async fn login(&self, req: Request<LoginReq>) -> Result<Response<LoginRes>, Status> {
        let req = req.into_inner();

        if EmailAddress::is_valid(&req.email) {
            return Err(Status::invalid_argument("Invalid Email Address"));
        };

        let db_user = match sqlx::query!(
            r#"
        SELECT *
        FROM Users
        WHERE email = $1
        "#,
            req.email
        )
        .fetch_one(&self.db)
        .await
        {
            Ok(user) => user,
            Err(sqlx::Error::RowNotFound) => {
                return Ok(Response::new(LoginRes {
                    token: None,
                    status: LoginResEnum::UserDoesntExist.into(),
                }));
            }

            Err(_err) => {
                error!("{_err}");
                return Err(Status::internal("Something went wrong"));
            }
        };

        if !verify(req.password, &db_user.hashedpass).map_err(|err| {
            error!("{err}");
            Status::internal("Error logging in")
        })? {
            return Ok(Response::new(LoginRes {
                token: None,
                status: LoginResEnum::WrongPassword.into(),
            }));
        }

        // if we get here, gen token and return back
        let token = self
            .auth
            .gen_token(&db_user.userpubid)
            .map_err(|_| Status::internal("Failed to login"))?;

        Ok(Response::new(LoginRes {
            token: Some(Token {
                access_token: token,
            }),
            status: LoginResEnum::LoginSuccess.into(),
        }))
    }
    async fn register(&self, req: Request<RegisterReq>) -> Result<Response<RegisterRes>, Status> {
        let req = req.into_inner();

        if !sqlx::query!(
            r#"
            SELECT *
            FROM Users
            WHERE Name = $1
            AND Suffix = $2"#,
            req.name,
            req.suffix,
        )
        .fetch_all(&self.db)
        .await
        .map_err(|err| {
            error!("{err}");
            Status::internal("xd")
        })?
        .is_empty()
        {
            return Ok(Response::new(RegisterRes {
                status: Existence::AlreadyExists.into(),
                token: None,
                created_user: None,
            }));
        }

        // alright time to do password hashing
        let hashed_pass = hash(req.password, DEFAULT_COST).map_err(|err| {
            error!("{err}");
            Status::internal("Failed to register user")
        })?;

        // if not taken, then insert into it
        let new_user = sqlx::query_as!(
            User,
            r#"
        INSERT INTO Users (UserPubId, Name, Suffix, HashedPass)
        VALUES ($1, $2, $3, $4)
        RETURNING UserPubId AS id, Name, Suffix;
        "#,
            Id::gen(),
            req.name,
            req.suffix,
            hashed_pass
        )
        .fetch_one(&self.db)
        .await
        .map_err(|err| {
            error!("{err}");
            Status::internal("xd")
        })?;

        let token = self
            .auth
            .gen_token(&new_user.id)
            .map_err(|_| Status::internal("Failed to create token."))?;

        Ok(Response::new(RegisterRes {
            created_user: Some(new_user.into()),
            token: Some(Token {
                access_token: token,
            }),
            status: Existence::Success.into(),
        }))
    }

    async fn verify_token(&self, req: Request<Token>) -> Result<Response<grpc::User>, Status> {
        match self.auth.validate_token(&req.into_inner().access_token) {
            Ok(data) => {
                let user = sqlx::query_as!(
                    User,
                    r#"
                SELECT UserPubId AS Id, Name, Suffix 
                FROM Users
                WHERE UserPubId=$1
                "#,
                    data.claims.sub
                )
                .fetch_one(&self.db)
                .await
                .map_err(|err| {
                    error!("{err}");
                    Status::internal("Internal database error")
                })?;

                return Ok(Response::new(user.into()));
            }

            Err(_) => Err(Status::unauthenticated("Failed to verify token.")),
        }
    }
}
