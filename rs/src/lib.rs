mod app;
mod cli;
mod client;
mod error;

pub mod files;
pub mod utils;

pub mod grpc {
    tonic::include_proto!("t2t");
}

// flatten
pub use client::*;
pub use error::{Error, Result};

pub use app::*;
pub use cli::*;
