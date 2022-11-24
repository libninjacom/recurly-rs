#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let account_id = "your account id";
    let coupon_id = "your coupon id";
    let response = client
        .create_coupon_redemption(account_id, coupon_id)
        .currency("your currency")
        .subscription_id("your subscription id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
