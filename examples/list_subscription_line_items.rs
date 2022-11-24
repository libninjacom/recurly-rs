#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let subscription_id = "your subscription id";
    let response = client
        .list_subscription_line_items(subscription_id)
        .ids(&["your ids"])
        .limit(1)
        .order("your order")
        .sort("your sort")
        .begin_time("your begin time")
        .end_time("your end time")
        .original(::serde_json::json!({}))
        .state("your state")
        .type_("your type")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
