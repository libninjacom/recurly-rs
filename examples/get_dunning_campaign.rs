#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let dunning_campaign_id = "your dunning campaign id";
    let response = client
        .get_dunning_campaign(dunning_campaign_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
