#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let subscription_id = "your subscription id";
    let add_on_id = "your add on id";
    let response = client
        .create_usage(subscription_id, add_on_id)
        .merchant_tag("your merchant tag")
        .amount(1.0)
        .recording_timestamp("your recording timestamp")
        .usage_timestamp("your usage timestamp")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
