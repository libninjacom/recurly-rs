#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let shipping_method_id = "your shipping method id";
    let response = client.get_shipping_method(shipping_method_id).send().await.unwrap();
    println!("{:#?}", response);
}
