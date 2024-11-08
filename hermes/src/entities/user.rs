use crate::grpc;

#[derive(sqlx::Type, Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub suffix: String,
}

impl From<User> for grpc::User {
    fn from(value: User) -> Self {
        grpc::User {
            id: value.id,
            name: value.name,
            suffix: value.suffix,
        }
    }
}

impl From<grpc::User> for User {
    fn from(value: grpc::User) -> Self {
        User {
            id: value.id,
            name: value.name,
            suffix: value.suffix,
        }
    }
}
