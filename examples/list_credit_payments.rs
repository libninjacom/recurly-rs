#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let response = client
        .list_credit_payments()
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
