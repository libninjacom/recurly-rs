#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let unique_coupon_code_id = "your unique coupon code id";
    let response = client
        .get_unique_coupon_code(unique_coupon_code_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
