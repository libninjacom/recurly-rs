#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let site_id = "your site id";
    let response = client.get_site(site_id).send().await.unwrap();
    println!("{:#?}", response);
}
