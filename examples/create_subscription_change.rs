#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let subscription_id = "your subscription id";
    let response = client
        .create_subscription_change(subscription_id)
        .timeframe("your timeframe")
        .plan_id("your plan id")
        .plan_code("your plan code")
        .unit_amount(1.0)
        .tax_inclusive(true)
        .quantity(1)
        .shipping(SubscriptionChangeShippingCreate {
            address_id: Some("your address id".to_owned()),
            amount: Some(1.0),
            method_code: Some("your method code".to_owned()),
            method_id: Some("your method id".to_owned()),
            address: Some(ShippingAddressCreate {
                region: Some("your region".to_owned()),
                company: Some("your company".to_owned()),
                email: Some("your email".to_owned()),
                nickname: Some("your nickname".to_owned()),
                phone: Some("your phone".to_owned()),
                street2: Some("your street 2".to_owned()),
                street1: "your street 1".to_owned(),
                country: "your country".to_owned(),
                vat_number: Some("your vat number".to_owned()),
                city: "your city".to_owned(),
                first_name: "your first name".to_owned(),
                postal_code: "your postal code".to_owned(),
                last_name: "your last name".to_owned(),
            }),
        })
        .coupon_codes(&["your coupon codes"])
        .add_ons(
            vec![
                SubscriptionAddOnUpdate { quantity : Some(1), add_on_source :
                Some("your add on source".to_owned()), unit_amount : Some(1.0), id :
                Some("your id".to_owned()), percentage_tiers :
                Some(vec![SubscriptionAddOnPercentageTier { usage_percentage :
                Some("your usage percentage".to_owned()), ending_amount : Some(1.0) }]),
                usage_percentage : Some(1.0), revenue_schedule_type :
                Some("your revenue schedule type".to_owned()), tiers :
                Some(vec![SubscriptionAddOnTier { unit_amount_decimal :
                Some("your unit amount decimal".to_owned()), ending_quantity : Some(1),
                usage_percentage : Some("your usage percentage".to_owned()), unit_amount
                : Some(1.0) }]), code : Some("your code".to_owned()), unit_amount_decimal
                : Some("your unit amount decimal".to_owned()) }
            ],
        )
        .collection_method("your collection method")
        .revenue_schedule_type("your revenue schedule type")
        .custom_fields(
            CustomFields(
                vec![
                    CustomField { value : "your value".to_owned(), name : "your name"
                    .to_owned() }
                ],
            ),
        )
        .po_number("your po number")
        .net_terms(1)
        .transaction_type("your transaction type")
        .billing_info(SubscriptionChangeBillingInfoCreate {
            subscription_change_billing_info: SubscriptionChangeBillingInfo {
                three_d_secure_action_result_token_id: Some(
                    "your three d secure action result token id".to_owned(),
                ),
            },
        })
        .ramp_intervals(
            vec![
                SubscriptionRampInterval { starting_billing_cycle : Some(1), unit_amount
                : Some(1) }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
