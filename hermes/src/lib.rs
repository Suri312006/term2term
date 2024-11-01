mod config;
pub mod db;
pub mod middleware;
pub mod services;
pub use config::*;
pub mod entities;
pub mod utils;
pub mod grpc {
    tonic::include_proto!("t2t");
}
