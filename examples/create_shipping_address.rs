#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateShippingAddressRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateShippingAddressRequired {
        first_name: "your first name",
        street1: "your street 1",
        postal_code: "your postal code",
        country: "your country",
        account_id: "your account id",
        last_name: "your last name",
        city: "your city",
    };
    let response = client
        .create_shipping_address(args)
        .nickname("your nickname")
        .company("your company")
        .email("your email")
        .vat_number("your vat number")
        .phone("your phone")
        .street2("your street 2")
        .region("your region")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
