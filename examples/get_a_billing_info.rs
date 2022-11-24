#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let account_id = "your account id";
    let billing_info_id = "your billing info id";
    let response = client
        .get_a_billing_info(account_id, billing_info_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
