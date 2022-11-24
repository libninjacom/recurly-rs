#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let account_id = "your account id";
    let currency = "your currency";
    let response = client
        .create_invoice(account_id, currency)
        .collection_method("your collection method")
        .charge_customer_notes("your charge customer notes")
        .credit_customer_notes("your credit customer notes")
        .net_terms(1)
        .po_number("your po number")
        .terms_and_conditions("your terms and conditions")
        .vat_reverse_charge_notes("your vat reverse charge notes")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
