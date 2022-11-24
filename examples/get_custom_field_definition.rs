#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let custom_field_definition_id = "your custom field definition id";
    let response = client
        .get_custom_field_definition(custom_field_definition_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
