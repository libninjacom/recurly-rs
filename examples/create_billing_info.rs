#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let account_id = "your account id";
    let response = client
        .create_billing_info(account_id)
        .token_id("your token id")
        .first_name("your first name")
        .last_name("your last name")
        .company("your company")
        .address(Address {
            country: Some("your country".to_owned()),
            city: Some("your city".to_owned()),
            phone: Some("your phone".to_owned()),
            street1: Some("your street 1".to_owned()),
            street2: Some("your street 2".to_owned()),
            region: Some("your region".to_owned()),
            postal_code: Some("your postal code".to_owned()),
        })
        .number("your number")
        .month("your month")
        .year("your year")
        .cvv("your cvv")
        .vat_number("your vat number")
        .ip_address("your ip address")
        .gateway_token("your gateway token")
        .gateway_code("your gateway code")
        .amazon_billing_agreement_id("your amazon billing agreement id")
        .paypal_billing_agreement_id("your paypal billing agreement id")
        .fraud_session_id("your fraud session id")
        .transaction_type("your transaction type")
        .three_d_secure_action_result_token_id(
            "your three d secure action result token id",
        )
        .iban("your iban")
        .name_on_account("your name on account")
        .account_number("your account number")
        .routing_number("your routing number")
        .sort_code("your sort code")
        .type_("your type")
        .account_type("your account type")
        .tax_identifier("your tax identifier")
        .tax_identifier_type("your tax identifier type")
        .primary_payment_method(true)
        .backup_payment_method(true)
        .external_hpp_type("your external hpp type")
        .online_banking_payment_type("your online banking payment type")
        .card_type("your card type")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
