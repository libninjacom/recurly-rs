#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let plan_code = "your plan code";
    let account = AccountCreate {
        shipping_addresses: vec![
            ShippingAddressCreate { company : Some("your company".to_owned()),
            postal_code : "your postal code".to_owned(), first_name : "your first name"
            .to_owned(), nickname : Some("your nickname".to_owned()), street1 :
            "your street 1".to_owned(), street2 : Some("your street 2".to_owned()),
            last_name : "your last name".to_owned(), email : Some("your email"
            .to_owned()), phone : Some("your phone".to_owned()), region :
            Some("your region".to_owned()), country : "your country".to_owned(), city :
            "your city".to_owned(), vat_number : Some("your vat number".to_owned()) }
        ],
        code: "your code".to_owned(),
        account_update: AccountUpdate {
            dunning_campaign_id: Some("your dunning campaign id".to_owned()),
            cc_emails: Some("your cc emails".to_owned()),
            preferred_locale: Some("your preferred locale".to_owned()),
            first_name: Some("your first name".to_owned()),
            address: Some(Address {
                region: Some("your region".to_owned()),
                country: Some("your country".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                street1: Some("your street 1".to_owned()),
                street2: Some("your street 2".to_owned()),
                city: Some("your city".to_owned()),
                phone: Some("your phone".to_owned()),
            }),
            tax_exempt: Some(true),
            username: Some("your username".to_owned()),
            vat_number: Some("your vat number".to_owned()),
            invoice_template_id: Some("your invoice template id".to_owned()),
            billing_info: Some(BillingInfoCreate {
                amazon_billing_agreement_id: Some(
                    "your amazon billing agreement id".to_owned(),
                ),
                tax_identifier_type: Some("your tax identifier type".to_owned()),
                name_on_account: Some("your name on account".to_owned()),
                account_type: Some("your account type".to_owned()),
                year: Some("your year".to_owned()),
                cvv: Some("your cvv".to_owned()),
                three_d_secure_action_result_token_id: Some(
                    "your three d secure action result token id".to_owned(),
                ),
                fraud_session_id: Some("your fraud session id".to_owned()),
                vat_number: Some("your vat number".to_owned()),
                tax_identifier: Some("your tax identifier".to_owned()),
                company: Some("your company".to_owned()),
                gateway_token: Some("your gateway token".to_owned()),
                paypal_billing_agreement_id: Some(
                    "your paypal billing agreement id".to_owned(),
                ),
                backup_payment_method: Some(true),
                external_hpp_type: Some("your external hpp type".to_owned()),
                month: Some("your month".to_owned()),
                token_id: Some("your token id".to_owned()),
                address: Some(Address {
                    region: Some("your region".to_owned()),
                    country: Some("your country".to_owned()),
                    postal_code: Some("your postal code".to_owned()),
                    street1: Some("your street 1".to_owned()),
                    street2: Some("your street 2".to_owned()),
                    city: Some("your city".to_owned()),
                    phone: Some("your phone".to_owned()),
                }),
                transaction_type: Some("your transaction type".to_owned()),
                iban: Some("your iban".to_owned()),
                card_type: Some("your card type".to_owned()),
                number: Some("your number".to_owned()),
                routing_number: Some("your routing number".to_owned()),
                sort_code: Some("your sort code".to_owned()),
                online_banking_payment_type: Some(
                    "your online banking payment type".to_owned(),
                ),
                type_: Some("your type".to_owned()),
                last_name: Some("your last name".to_owned()),
                primary_payment_method: Some(true),
                first_name: Some("your first name".to_owned()),
                ip_address: Some("your ip address".to_owned()),
                gateway_code: Some("your gateway code".to_owned()),
                account_number: Some("your account number".to_owned()),
            }),
            custom_fields: Some(
                CustomFields(
                    vec![
                        CustomField { value : "your value".to_owned(), name : "your name"
                        .to_owned() }
                    ],
                ),
            ),
            exemption_certificate: Some("your exemption certificate".to_owned()),
            parent_account_code: Some("your parent account code".to_owned()),
            bill_to: Some("your bill to".to_owned()),
            parent_account_id: Some("your parent account id".to_owned()),
            company: Some("your company".to_owned()),
            transaction_type: Some("your transaction type".to_owned()),
            email: Some("your email".to_owned()),
            last_name: Some("your last name".to_owned()),
        },
        acquisition: AccountAcquisitionUpdate {
            campaign: Some("your campaign".to_owned()),
            cost: Some(::serde_json::json!({})),
            subchannel: Some("your subchannel".to_owned()),
            channel: Some("your channel".to_owned()),
        },
    };
    let currency = "your currency";
    let response = client
        .create_subscription(plan_code, account, currency)
        .plan_id("your plan id")
        .billing_info_id("your billing info id")
        .shipping(SubscriptionShippingCreate {
            method_id: Some("your method id".to_owned()),
            address_id: Some("your address id".to_owned()),
            method_code: Some("your method code".to_owned()),
            address: Some(ShippingAddressCreate {
                company: Some("your company".to_owned()),
                postal_code: "your postal code".to_owned(),
                first_name: "your first name".to_owned(),
                nickname: Some("your nickname".to_owned()),
                street1: "your street 1".to_owned(),
                street2: Some("your street 2".to_owned()),
                last_name: "your last name".to_owned(),
                email: Some("your email".to_owned()),
                phone: Some("your phone".to_owned()),
                region: Some("your region".to_owned()),
                country: "your country".to_owned(),
                city: "your city".to_owned(),
                vat_number: Some("your vat number".to_owned()),
            }),
            amount: Some(1.0),
        })
        .collection_method("your collection method")
        .unit_amount(1.0)
        .tax_inclusive(true)
        .quantity(1)
        .add_ons(
            vec![
                SubscriptionAddOnCreate { revenue_schedule_type :
                Some("your revenue schedule type".to_owned()), code : "your code"
                .to_owned(), tiers : Some(vec![SubscriptionAddOnTier { usage_percentage :
                Some("your usage percentage".to_owned()), unit_amount : Some(1.0),
                ending_quantity : Some(1), unit_amount_decimal :
                Some("your unit amount decimal".to_owned()) }]), quantity : Some(1),
                unit_amount : Some(1.0), add_on_source : Some("your add on source"
                .to_owned()), percentage_tiers :
                Some(vec![SubscriptionAddOnPercentageTier { usage_percentage :
                Some("your usage percentage".to_owned()), ending_amount : Some(1.0) }]),
                usage_percentage : Some(1.0), unit_amount_decimal :
                Some("your unit amount decimal".to_owned()) }
            ],
        )
        .coupon_codes(&["your coupon codes"])
        .custom_fields(
            CustomFields(
                vec![
                    CustomField { value : "your value".to_owned(), name : "your name"
                    .to_owned() }
                ],
            ),
        )
        .trial_ends_at("your trial ends at")
        .starts_at("your starts at")
        .next_bill_date("your next bill date")
        .total_billing_cycles(1)
        .renewal_billing_cycles(1)
        .auto_renew(true)
        .ramp_intervals(
            vec![
                SubscriptionRampInterval { starting_billing_cycle : Some(1), unit_amount
                : Some(1) }
            ],
        )
        .revenue_schedule_type("your revenue schedule type")
        .terms_and_conditions("your terms and conditions")
        .customer_notes("your customer notes")
        .credit_customer_notes("your credit customer notes")
        .po_number("your po number")
        .net_terms(1)
        .transaction_type("your transaction type")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
