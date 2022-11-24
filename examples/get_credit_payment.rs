#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let credit_payment_id = "your credit payment id";
    let response = client.get_credit_payment(credit_payment_id).send().await.unwrap();
    println!("{:#?}", response);
}
