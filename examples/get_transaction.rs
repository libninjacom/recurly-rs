#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let transaction_id = "your transaction id";
    let response = client.get_transaction(transaction_id).send().await.unwrap();
    println!("{:#?}", response);
}
