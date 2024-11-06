use crate::grpc;

use super::user::User;

#[derive(sqlx::Type, Debug, Clone)]
pub struct Conversation {
    pub id: String,
    pub participants: Vec<User>,
}

impl From<Conversation> for grpc::Convo {
    fn from(value: Conversation) -> Self {
        todo!()
    }
}
