#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let invoice_template_id = "your invoice template id";
    let response = client
        .list_invoice_template_accounts(invoice_template_id)
        .ids(&["your ids"])
        .limit(1)
        .order("your order")
        .sort("your sort")
        .begin_time("your begin time")
        .end_time("your end time")
        .email("your email")
        .subscriber(true)
        .past_due(::serde_json::json!({}))
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
