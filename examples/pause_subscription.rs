#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let subscription_id = "your subscription id";
    let remaining_pause_cycles = 1;
    let response = client
        .pause_subscription(subscription_id, remaining_pause_cycles)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
