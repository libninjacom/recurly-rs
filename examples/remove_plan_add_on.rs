#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let plan_id = "your plan id";
    let add_on_id = "your add on id";
    let response = client.remove_plan_add_on(plan_id, add_on_id).send().await.unwrap();
    println!("{:#?}", response);
}
