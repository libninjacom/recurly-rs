#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let coupon_id = "your coupon id";
    let response = client
        .list_unique_coupon_codes(coupon_id)
        .ids(&["your ids"])
        .limit(1)
        .order("your order")
        .sort("your sort")
        .begin_time("your begin time")
        .end_time("your end time")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
