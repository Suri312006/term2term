mod conversation;
mod message;
mod user;

mod grpc {
    tonic::include_proto!("t2t");
}

use grpc::*;

pub use conversation::*;
pub use message::*;
pub use user::*;
