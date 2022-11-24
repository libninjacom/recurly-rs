#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let name = "your name";
    let display_name = "your display name";
    let response = client
        .create_measured_unit(name, display_name)
        .description("your description")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
