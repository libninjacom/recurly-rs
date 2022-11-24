#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let invoice_template_id = "your invoice template id";
    let response = client
        .get_invoice_template(invoice_template_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
