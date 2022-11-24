#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let invoice_id = "your invoice id";
    let response = client.void_invoice(invoice_id).send().await.unwrap();
    println!("{:#?}", response);
}
