#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let account_id = "your account id";
    let account_note_id = "your account note id";
    let response = client
        .get_account_note(account_id, account_note_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
