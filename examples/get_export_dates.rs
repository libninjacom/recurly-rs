#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let response = client.get_export_dates().send().await.unwrap();
    println!("{:#?}", response);
}
