#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let account_id = "your account id";
    let shipping_address_id = "your shipping address id";
    let response = client
        .get_shipping_address(account_id, shipping_address_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
