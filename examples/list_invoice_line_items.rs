#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let invoice_id = "your invoice id";
    let response = client
        .list_invoice_line_items(invoice_id)
        .ids(&["your ids"])
        .limit(1)
        .order("your order")
        .sort("your sort")
        .begin_time("your begin time")
        .end_time("your end time")
        .original(::serde_json::json!({}))
        .state("your state")
        .type_("your type")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
