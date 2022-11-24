#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let coupon_id = "your coupon id";
    let response = client
        .generate_unique_coupon_codes(coupon_id)
        .number_of_unique_codes(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
