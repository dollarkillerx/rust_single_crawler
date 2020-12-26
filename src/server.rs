use super::*;
use single_crawler_rpc::single_crawler_server::SingleCrawler;
use single_crawler_rpc::{CrawlerReq, CrawlerResp};
use reqwest;

#[derive(Debug, Default)]
pub struct SingleCrawlerService {}

#[tonic::async_trait]
impl SingleCrawler for SingleCrawlerService {
    async fn single_crawler(&self, request: Request<CrawlerReq>) -> std::result::Result<Response<CrawlerResp>, Status> {

        let req: CrawlerReq = request.into_inner();
        let r = reqwest::get(&req.url).await.map_err(| err|{ Status::new(Code::Ok, format!("{:?}",err)) })?.bytes().await.map_err(| err|{ Status::new(Code::Ok, format!("{:?}",err)) })?;

        let response = CrawlerResp {
            body: r.to_vec(),
            http_code: 200,
        };

        Ok(Response::new(response))
    }
}

// println!("request: {:?}", request);
// println!("remote_addr: {:?}", request.remote_addr());
// println!("metadata: {:?}", request.metadata());

// let r = match reqwest::get(&req.url).await {
// Ok(data) => {
// match data.bytes().await {
// Ok(r) => r,
// Err(err) => {
// return std::result::Result::Err(Status::new(Code::Ok,format!("{:?}",err)))
// }
// }
// }
// Err(err) => {
// return std::result::Result::Err(Status::new(Code::Ok,format!("{:?}",err)))
// }
// };