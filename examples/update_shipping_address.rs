#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let account_id = "your account id";
    let shipping_address_id = "your shipping address id";
    let response = client
        .update_shipping_address(account_id, shipping_address_id)
        .id("your id")
        .nickname("your nickname")
        .first_name("your first name")
        .last_name("your last name")
        .company("your company")
        .email("your email")
        .vat_number("your vat number")
        .phone("your phone")
        .street1("your street 1")
        .street2("your street 2")
        .city("your city")
        .region("your region")
        .postal_code("your postal code")
        .country("your country")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
