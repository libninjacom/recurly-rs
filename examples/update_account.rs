#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let account_id = "your account id";
    let response = client
        .update_account(account_id)
        .username("your username")
        .email("your email")
        .preferred_locale("your preferred locale")
        .cc_emails("your cc emails")
        .first_name("your first name")
        .last_name("your last name")
        .company("your company")
        .vat_number("your vat number")
        .tax_exempt(true)
        .exemption_certificate("your exemption certificate")
        .parent_account_code("your parent account code")
        .parent_account_id("your parent account id")
        .bill_to("your bill to")
        .transaction_type("your transaction type")
        .dunning_campaign_id("your dunning campaign id")
        .invoice_template_id("your invoice template id")
        .address(Address {
            phone: Some("your phone".to_owned()),
            postal_code: Some("your postal code".to_owned()),
            city: Some("your city".to_owned()),
            street1: Some("your street 1".to_owned()),
            region: Some("your region".to_owned()),
            street2: Some("your street 2".to_owned()),
            country: Some("your country".to_owned()),
        })
        .billing_info(BillingInfoCreate {
            transaction_type: Some("your transaction type".to_owned()),
            month: Some("your month".to_owned()),
            vat_number: Some("your vat number".to_owned()),
            sort_code: Some("your sort code".to_owned()),
            primary_payment_method: Some(true),
            last_name: Some("your last name".to_owned()),
            amazon_billing_agreement_id: Some(
                "your amazon billing agreement id".to_owned(),
            ),
            card_type: Some("your card type".to_owned()),
            backup_payment_method: Some(true),
            gateway_token: Some("your gateway token".to_owned()),
            three_d_secure_action_result_token_id: Some(
                "your three d secure action result token id".to_owned(),
            ),
            account_number: Some("your account number".to_owned()),
            fraud_session_id: Some("your fraud session id".to_owned()),
            first_name: Some("your first name".to_owned()),
            gateway_code: Some("your gateway code".to_owned()),
            name_on_account: Some("your name on account".to_owned()),
            tax_identifier: Some("your tax identifier".to_owned()),
            cvv: Some("your cvv".to_owned()),
            routing_number: Some("your routing number".to_owned()),
            number: Some("your number".to_owned()),
            online_banking_payment_type: Some(
                "your online banking payment type".to_owned(),
            ),
            account_type: Some("your account type".to_owned()),
            paypal_billing_agreement_id: Some(
                "your paypal billing agreement id".to_owned(),
            ),
            address: Some(Address {
                phone: Some("your phone".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                city: Some("your city".to_owned()),
                street1: Some("your street 1".to_owned()),
                region: Some("your region".to_owned()),
                street2: Some("your street 2".to_owned()),
                country: Some("your country".to_owned()),
            }),
            year: Some("your year".to_owned()),
            tax_identifier_type: Some("your tax identifier type".to_owned()),
            token_id: Some("your token id".to_owned()),
            type_: Some("your type".to_owned()),
            company: Some("your company".to_owned()),
            external_hpp_type: Some("your external hpp type".to_owned()),
            ip_address: Some("your ip address".to_owned()),
            iban: Some("your iban".to_owned()),
        })
        .custom_fields(
            CustomFields(
                vec![
                    CustomField { name : "your name".to_owned(), value : "your value"
                    .to_owned() }
                ],
            ),
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
