#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let response = client
        .list_invoice_templates()
        .sort("your sort")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
