pub mod server;

use tonic;
pub use single_crawler_rpc::single_crawler_server::SingleCrawlerServer;
pub use tonic::{transport::Server, Request, Response, Status, Code};

extern crate rand;

pub mod single_crawler_rpc {
    tonic::include_proto!("single_crawler");
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;