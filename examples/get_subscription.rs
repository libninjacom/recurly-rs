#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let subscription_id = "your subscription id";
    let response = client.get_subscription(subscription_id).send().await.unwrap();
    println!("{:#?}", response);
}
