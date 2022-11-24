#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let invoice_id = "your invoice id";
    let response = client
        .list_invoice_coupon_redemptions(invoice_id)
        .ids(&["your ids"])
        .sort("your sort")
        .begin_time("your begin time")
        .end_time("your end time")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
