#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let coupon_id = "your coupon id";
    let response = client.get_coupon(coupon_id).send().await.unwrap();
    println!("{:#?}", response);
}
