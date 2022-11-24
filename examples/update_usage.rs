#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let usage_id = "your usage id";
    let response = client
        .update_usage(usage_id)
        .merchant_tag("your merchant tag")
        .amount(1.0)
        .recording_timestamp("your recording timestamp")
        .usage_timestamp("your usage timestamp")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
