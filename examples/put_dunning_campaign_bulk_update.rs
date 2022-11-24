#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let dunning_campaign_id = "your dunning campaign id";
    let response = client
        .put_dunning_campaign_bulk_update(dunning_campaign_id)
        .plan_codes(&["your plan codes"])
        .plan_ids(&["your plan ids"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
