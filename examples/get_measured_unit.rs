#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let measured_unit_id = "your measured unit id";
    let response = client.get_measured_unit(measured_unit_id).send().await.unwrap();
    println!("{:#?}", response);
}
