use super::*;
use single_crawler_rpc::single_crawler_server::SingleCrawler;
use single_crawler_rpc::{CrawlerReq, CrawlerResp};

#[derive(Debug, Default)]
pub struct SingleCrawlerService {}

#[tonic::async_trait]
impl SingleCrawler for SingleCrawlerService {
    async fn single_crawler(&self, request: Request<CrawlerReq>) -> std::result::Result<Response<CrawlerResp>, Status> {
        println!("request: {:?}", request);
        println!("remote_addr: {:?}", request.remote_addr());
        println!("metadata: {:?}", request.metadata());

        let response = CrawlerResp {
            body: "sd".as_bytes().to_vec(),
            http_code: 200,
        };

        Ok(Response::new(response))
    }
}