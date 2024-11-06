mod config;
pub use config::*;
pub mod db;
pub mod entities;
pub mod middleware;
pub mod services;
pub mod utils;
pub mod grpc {
    tonic::include_proto!("t2t");
}
