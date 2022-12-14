#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let invoice_id = "your invoice id";
    let type_ = "your type";
    let response = client
        .refund_invoice(invoice_id, type_)
        .amount(1.0)
        .line_items(
            vec![
                LineItemRefund { quantity_decimal : Some("your quantity decimal"
                .to_owned()), quantity : Some(1), id : Some("your id".to_owned()),
                prorate : Some(true) }
            ],
        )
        .refund_method("your refund method")
        .credit_customer_notes("your credit customer notes")
        .external_refund(::serde_json::json!({}))
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
