#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let account_id = "your account id";
    let response = client
        .list_entitlements(account_id)
        .state("your state")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
