mod error;

pub mod cli;
pub mod files;
pub mod utils;

pub mod grpc {
    tonic::include_proto!("t2t");
}

// flatten
pub use error::{Error, Result};

