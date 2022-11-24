#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let response = client
        .list_sites()
        .ids(&["your ids"])
        .limit(1)
        .order("your order")
        .sort("your sort")
        .state("your state")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
