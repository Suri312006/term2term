mod error;
mod cli;
mod client;

pub mod files;
pub mod utils;

pub mod grpc {
    tonic::include_proto!("t2t");
}

// flatten
pub use error::{Error, Result};
pub use client::*;

pub use cli::*;
