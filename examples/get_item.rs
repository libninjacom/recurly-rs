#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let item_id = "your item id";
    let response = client.get_item(item_id).send().await.unwrap();
    println!("{:#?}", response);
}
