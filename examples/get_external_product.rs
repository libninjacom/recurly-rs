#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let external_product_id = "your external product id";
    let response = client
        .get_external_product(external_product_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
