use sea_orm::{ActiveModelTrait, EntityTrait, TryIntoModel};
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
            name: ActiveValue::Set("Surendra".to_owned()),
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
