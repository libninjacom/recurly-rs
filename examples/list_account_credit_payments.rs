#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let account_id = "your account id";
    let response = client
        .list_account_credit_payments(account_id)
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
