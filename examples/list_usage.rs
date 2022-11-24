#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let subscription_id = "your subscription id";
    let add_on_id = "your add on id";
    let response = client
        .list_usage(subscription_id, add_on_id)
        .ids(&["your ids"])
        .limit(1)
        .order("your order")
        .sort("your sort")
        .begin_time("your begin time")
        .end_time("your end time")
        .billing_status("your billing status")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
