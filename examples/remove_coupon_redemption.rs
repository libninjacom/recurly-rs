#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let account_id = "your account id";
    let response = client.remove_coupon_redemption(account_id).send().await.unwrap();
    println!("{:#?}", response);
}
