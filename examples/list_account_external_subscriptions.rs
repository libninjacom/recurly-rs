#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let account_id = "your account id";
    let response = client
        .list_account_external_subscriptions(account_id)
        .sort("your sort")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
