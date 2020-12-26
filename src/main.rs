use single_crawler::*;
use std::env;

#[tokio::main]
async fn main() ->Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./xxx 0.0.0.0:8081");
    }
    let addr = args[1].parse()?;
    let ser = server::SingleCrawlerService::default();
    println!("Rust Single Crawler listening on {}",addr);

    Server::builder()
        .add_service(SingleCrawlerServer::new(ser))
        .serve(addr)
        .await?;
    Ok(())
}
