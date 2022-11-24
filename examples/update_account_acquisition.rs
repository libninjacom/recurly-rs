#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let account_id = "your account id";
    let response = client
        .update_account_acquisition(account_id)
        .cost(::serde_json::json!({}))
        .channel("your channel")
        .subchannel("your subchannel")
        .campaign("your campaign")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
