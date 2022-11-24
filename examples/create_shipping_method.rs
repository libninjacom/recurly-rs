#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let code = "your code";
    let name = "your name";
    let response = client
        .create_shipping_method(code, name)
        .accounting_code("your accounting code")
        .tax_code("your tax code")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
