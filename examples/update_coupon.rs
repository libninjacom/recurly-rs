#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let coupon_id = "your coupon id";
    let response = client
        .update_coupon(coupon_id)
        .name("your name")
        .max_redemptions(1)
        .max_redemptions_per_account(1)
        .hosted_description("your hosted description")
        .invoice_description("your invoice description")
        .redeem_by_date("your redeem by date")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
