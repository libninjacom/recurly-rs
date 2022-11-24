#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let usage_id = "your usage id";
    let response = client.get_usage(usage_id).send().await.unwrap();
    println!("{:#?}", response);
}
