use single_crawler::*;
use single_crawler_rpc::single_crawler_client::SingleCrawlerClient;
use single_crawler_rpc::{CrawlerReq, CrawlerResp};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = SingleCrawlerClient::connect("http://127.0.0.1:8081").await?;

    println!("client: {:?}", client);

    let request = Request::new(CrawlerReq {
        url: "http://www.baidu.com".to_string(),
        timeout: 600,
    });

    let response = client.single_crawler(request).await?;

    // let p:Vec<u8>  = Vec::new();
    // String::from_utf8(p);
    let data: CrawlerResp = response.into_inner();
    println!("resp: {:?}", String::from_utf8(data.body));

    Ok(())
}