#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let currency = "your currency";
    let account = AccountPurchase {
        id: "your id".to_owned(),
        code: "your code".to_owned(),
        acquisition: AccountAcquisitionUpdate {
            channel: Some("your channel".to_owned()),
            subchannel: Some("your subchannel".to_owned()),
            campaign: Some("your campaign".to_owned()),
            cost: Some(::serde_json::json!({})),
        },
        account_update: AccountUpdate {
            tax_exempt: Some(true),
            invoice_template_id: Some("your invoice template id".to_owned()),
            parent_account_code: Some("your parent account code".to_owned()),
            email: Some("your email".to_owned()),
            cc_emails: Some("your cc emails".to_owned()),
            first_name: Some("your first name".to_owned()),
            vat_number: Some("your vat number".to_owned()),
            preferred_locale: Some("your preferred locale".to_owned()),
            parent_account_id: Some("your parent account id".to_owned()),
            address: Some(Address {
                city: Some("your city".to_owned()),
                phone: Some("your phone".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                country: Some("your country".to_owned()),
                street2: Some("your street 2".to_owned()),
                street1: Some("your street 1".to_owned()),
                region: Some("your region".to_owned()),
            }),
            dunning_campaign_id: Some("your dunning campaign id".to_owned()),
            last_name: Some("your last name".to_owned()),
            transaction_type: Some("your transaction type".to_owned()),
            billing_info: Some(BillingInfoCreate {
                company: Some("your company".to_owned()),
                number: Some("your number".to_owned()),
                amazon_billing_agreement_id: Some(
                    "your amazon billing agreement id".to_owned(),
                ),
                token_id: Some("your token id".to_owned()),
                year: Some("your year".to_owned()),
                ip_address: Some("your ip address".to_owned()),
                cvv: Some("your cvv".to_owned()),
                online_banking_payment_type: Some(
                    "your online banking payment type".to_owned(),
                ),
                gateway_code: Some("your gateway code".to_owned()),
                transaction_type: Some("your transaction type".to_owned()),
                iban: Some("your iban".to_owned()),
                account_number: Some("your account number".to_owned()),
                routing_number: Some("your routing number".to_owned()),
                address: Some(Address {
                    city: Some("your city".to_owned()),
                    phone: Some("your phone".to_owned()),
                    postal_code: Some("your postal code".to_owned()),
                    country: Some("your country".to_owned()),
                    street2: Some("your street 2".to_owned()),
                    street1: Some("your street 1".to_owned()),
                    region: Some("your region".to_owned()),
                }),
                vat_number: Some("your vat number".to_owned()),
                last_name: Some("your last name".to_owned()),
                name_on_account: Some("your name on account".to_owned()),
                first_name: Some("your first name".to_owned()),
                tax_identifier: Some("your tax identifier".to_owned()),
                primary_payment_method: Some(true),
                account_type: Some("your account type".to_owned()),
                tax_identifier_type: Some("your tax identifier type".to_owned()),
                external_hpp_type: Some("your external hpp type".to_owned()),
                sort_code: Some("your sort code".to_owned()),
                three_d_secure_action_result_token_id: Some(
                    "your three d secure action result token id".to_owned(),
                ),
                backup_payment_method: Some(true),
                type_: Some("your type".to_owned()),
                gateway_token: Some("your gateway token".to_owned()),
                paypal_billing_agreement_id: Some(
                    "your paypal billing agreement id".to_owned(),
                ),
                card_type: Some("your card type".to_owned()),
                fraud_session_id: Some("your fraud session id".to_owned()),
                month: Some("your month".to_owned()),
            }),
            bill_to: Some("your bill to".to_owned()),
            username: Some("your username".to_owned()),
            exemption_certificate: Some("your exemption certificate".to_owned()),
            custom_fields: Some(
                CustomFields(
                    vec![
                        CustomField { name : "your name".to_owned(), value : "your value"
                        .to_owned() }
                    ],
                ),
            ),
            company: Some("your company".to_owned()),
        },
    };
    let response = client
        .preview_purchase(currency, account)
        .billing_info_id("your billing info id")
        .collection_method("your collection method")
        .po_number("your po number")
        .net_terms(1)
        .terms_and_conditions("your terms and conditions")
        .customer_notes("your customer notes")
        .vat_reverse_charge_notes("your vat reverse charge notes")
        .credit_customer_notes("your credit customer notes")
        .gateway_code("your gateway code")
        .shipping(::serde_json::json!({}))
        .line_items(
            vec![
                LineItemCreate { end_date : Some("your end date".to_owned()),
                avalara_transaction_type : Some(1), item_id : Some("your item id"
                .to_owned()), currency : "your currency".to_owned(), tax_exempt :
                Some(true), start_date : Some("your start date".to_owned()), unit_amount
                : 1.0, quantity : Some(1), revenue_schedule_type :
                Some("your revenue schedule type".to_owned()), tax_inclusive :
                Some(true), credit_reason_code : Some("your credit reason code"
                .to_owned()), accounting_code : Some("your accounting code".to_owned()),
                item_code : Some("your item code".to_owned()), description :
                Some("your description".to_owned()), avalara_service_type : Some(1),
                tax_code : Some("your tax code".to_owned()), product_code :
                Some("your product code".to_owned()), origin : Some("your origin"
                .to_owned()), type_ : "your type".to_owned() }
            ],
        )
        .subscriptions(
            vec![
                SubscriptionPurchase { custom_fields : Some(CustomFields(vec![CustomField
                { name : "your name".to_owned(), value : "your value".to_owned() }])),
                tax_inclusive : Some(true), quantity : Some(1), trial_ends_at :
                Some("your trial ends at".to_owned()), revenue_schedule_type :
                Some("your revenue schedule type".to_owned()), add_ons :
                Some(vec![SubscriptionAddOnCreate { unit_amount_decimal :
                Some("your unit amount decimal".to_owned()), add_on_source :
                Some("your add on source".to_owned()), quantity : Some(1), tiers :
                Some(vec![SubscriptionAddOnTier { usage_percentage :
                Some("your usage percentage".to_owned()), ending_quantity : Some(1),
                unit_amount : Some(1.0), unit_amount_decimal :
                Some("your unit amount decimal".to_owned()) }]), percentage_tiers :
                Some(vec![SubscriptionAddOnPercentageTier { usage_percentage :
                Some("your usage percentage".to_owned()), ending_amount : Some(1.0) }]),
                unit_amount : Some(1.0), usage_percentage : Some(1.0),
                revenue_schedule_type : Some("your revenue schedule type".to_owned()),
                code : "your code".to_owned() }]), unit_amount : Some(1.0),
                renewal_billing_cycles : Some(1), total_billing_cycles : Some(1),
                starts_at : Some("your starts at".to_owned()), auto_renew : Some(true),
                ramp_intervals : Some(vec![SubscriptionRampInterval { unit_amount :
                Some(1), starting_billing_cycle : Some(1) }]), plan_code :
                "your plan code".to_owned(), shipping : Some(SubscriptionShippingPurchase
                { method_id : Some("your method id".to_owned()), method_code :
                Some("your method code".to_owned()), amount : Some(1.0) }),
                next_bill_date : Some("your next bill date".to_owned()), plan_id :
                Some("your plan id".to_owned()) }
            ],
        )
        .coupon_codes(&["your coupon codes"])
        .gift_card_redemption_code("your gift card redemption code")
        .transaction_type("your transaction type")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
