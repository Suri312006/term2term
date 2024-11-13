mod client;
pub use client::*;

pub mod grpc {
    tonic::include_proto!("t2t");
}
