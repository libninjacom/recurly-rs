#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let add_on_id = "your add on id";
    let response = client.get_add_on(add_on_id).send().await.unwrap();
    println!("{:#?}", response);
}
