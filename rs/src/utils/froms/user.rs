

use crate::{
    files::config::ConfigUser,
    grpc::User,
};
impl From<User> for ConfigUser {
    fn from(value: User) -> Self {
        ConfigUser {
            username: value.name,
            id: value.id,
        }
    }
}
impl From<ConfigUser> for User {
    fn from(value: ConfigUser) -> Self {
        User {
            name: value.username,
            id: value.id,
        }
    }
}
