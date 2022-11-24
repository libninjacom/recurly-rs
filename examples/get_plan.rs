#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let plan_id = "your plan id";
    let response = client.get_plan(plan_id).send().await.unwrap();
    println!("{:#?}", response);
}
