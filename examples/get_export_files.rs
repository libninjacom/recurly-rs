#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let export_date = "your export date";
    let response = client.get_export_files(export_date).send().await.unwrap();
    println!("{:#?}", response);
}
