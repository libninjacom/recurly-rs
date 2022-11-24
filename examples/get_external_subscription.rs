#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let external_subscription_id = "your external subscription id";
    let response = client
        .get_external_subscription(external_subscription_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
