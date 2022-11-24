#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let account_id = "your account id";
    let response = client
        .verify_billing_info(account_id)
        .gateway_code("your gateway code")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
