#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let measured_unit_id = "your measured unit id";
    let response = client
        .update_measured_unit(measured_unit_id)
        .name("your name")
        .display_name("your display name")
        .description("your description")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
