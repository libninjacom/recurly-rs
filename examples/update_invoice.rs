#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let invoice_id = "your invoice id";
    let response = client
        .update_invoice(invoice_id)
        .po_number("your po number")
        .vat_reverse_charge_notes("your vat reverse charge notes")
        .terms_and_conditions("your terms and conditions")
        .customer_notes("your customer notes")
        .net_terms(1)
        .address(InvoiceAddress {
            name_on_account: Some("your name on account".to_owned()),
            company: Some("your company".to_owned()),
        })
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
