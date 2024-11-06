use crate::grpc;

use super::{conversation::Conversation, user::User};

//message Msg {
//  string id = 1;
//  Convo convo = 2;
//  User author = 3;
//  User recipient = 4;
//  string body = 5;
//  uint64 unix_time = 6;
//  bool isRead = 7;
//}

//TODO: finish this
#[derive(sqlx::Type, Debug, Clone)]
pub struct Message {
    id: String,
    convo: Conversation,
    author: User,
}

impl From<Message> for grpc::Msg {
    fn from(value: Message) -> Self {
        todo!()
    }
}
