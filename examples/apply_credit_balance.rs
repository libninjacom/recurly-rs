#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let site_id = "your site id";
    let invoice_id = "your invoice id";
    let response = client
        .apply_credit_balance(site_id, invoice_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
