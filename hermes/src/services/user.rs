use sea_orm::{ActiveModelTrait, TryIntoModel};
use sea_orm::{ActiveValue, DatabaseConnection};
use tonic::{Request, Response, Status};

use crate::entities::user;
use crate::utils::Id;

use super::{user_service_server::UserService, NewUserReq, UserInfo};

#[derive(Default)]
pub struct UserServer {
    pub db: DatabaseConnection,
}

#[tonic::async_trait]
impl UserService for UserServer {
    async fn create(&self, request: Request<NewUserReq>) -> Result<Response<UserInfo>, Status> {
        let x = user::ActiveModel {
            name: ActiveValue::Set(request.into_inner().name),
            pub_id: ActiveValue::Set(Id::gen()),
            ..Default::default()
        }
        .save(&self.db)
        .await
        .unwrap()
        .try_into_model()
        .unwrap();

        println!("{:#?}", x);

        Ok(Response::new(UserInfo {
            name: x.name,
            id: x.pub_id,
        }))
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use sea_orm::{prelude::*, Database, DatabaseBackend, MockDatabase, MockExecResult};
    use tonic::IntoRequest;

    use crate::{
        entities::user,
        services::{user_service_server::UserService, NewUserReq, UserServer},
    };

    #[tokio::test]
    async fn create_user() -> Result<()> {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![[user::Model {
                name: "Surendra".to_owned(),
                pub_id: "123456789012345678901".to_owned(),
                id: 1,
                ..Default::default()
            }]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();

        let server = UserServer { db };

        let res = server
            .create(
                NewUserReq {
                    name: "Surendra".to_owned(),
                }
                .into_request(),
            )
            .await?
            .into_inner();

        let transaction_lg = server.db.into_transaction_log();
        println!("{transaction_lg:#?}");
        assert_eq!(res.name.as_str(), "Surendra");
        assert_eq!(res.id.as_str().len(), 21);
        assert!(!transaction_lg.is_empty());
        Ok(())
    }
}
