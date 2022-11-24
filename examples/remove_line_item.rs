#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let line_item_id = "your line item id";
    let response = client.remove_line_item(line_item_id).send().await.unwrap();
    println!("{:#?}", response);
}
