#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let invoice_id = "your invoice id";
    let response = client
        .collect_invoice(invoice_id)
        .three_d_secure_action_result_token_id(
            "your three d secure action result token id",
        )
        .transaction_type("your transaction type")
        .billing_info_id("your billing info id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
