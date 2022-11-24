#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let shipping_method_id = "your shipping method id";
    let response = client
        .update_shipping_method(shipping_method_id)
        .code("your code")
        .name("your name")
        .accounting_code("your accounting code")
        .tax_code("your tax code")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
