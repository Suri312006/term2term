use crate::{files::config::ConfigUser, grpc::UserInfo};

impl From<UserInfo> for ConfigUser {
    fn from(value: UserInfo) -> Self {
        ConfigUser {
            username: value.name,
            id: value.id,
        }
    }
}
impl From<ConfigUser> for UserInfo {
    fn from(value: ConfigUser) -> Self {
        UserInfo {
            name: value.username,
            id: value.id,
        }
    }
}
