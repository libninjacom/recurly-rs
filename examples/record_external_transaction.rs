#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let invoice_id = "your invoice id";
    let response = client
        .record_external_transaction(invoice_id)
        .payment_method("your payment method")
        .description("your description")
        .amount(1.0)
        .collected_at("your collected at")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
