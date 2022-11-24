#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let subscription_id = "your subscription id";
    let response = client
        .update_subscription(subscription_id)
        .collection_method("your collection method")
        .custom_fields(
            CustomFields(
                vec![
                    CustomField { value : "your value".to_owned(), name : "your name"
                    .to_owned() }
                ],
            ),
        )
        .remaining_billing_cycles(1)
        .renewal_billing_cycles(1)
        .auto_renew(true)
        .next_bill_date("your next bill date")
        .revenue_schedule_type("your revenue schedule type")
        .terms_and_conditions("your terms and conditions")
        .customer_notes("your customer notes")
        .po_number("your po number")
        .net_terms(1)
        .gateway_code("your gateway code")
        .tax_inclusive(true)
        .shipping(SubscriptionShippingUpdate {
            address: Some(ShippingAddressCreate {
                nickname: Some("your nickname".to_owned()),
                phone: Some("your phone".to_owned()),
                vat_number: Some("your vat number".to_owned()),
                last_name: "your last name".to_owned(),
                street2: Some("your street 2".to_owned()),
                city: "your city".to_owned(),
                company: Some("your company".to_owned()),
                postal_code: "your postal code".to_owned(),
                country: "your country".to_owned(),
                street1: "your street 1".to_owned(),
                email: Some("your email".to_owned()),
                region: Some("your region".to_owned()),
                first_name: "your first name".to_owned(),
            }),
            address_id: Some("your address id".to_owned()),
            object: Some("your object".to_owned()),
        })
        .billing_info_id("your billing info id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
