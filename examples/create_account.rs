#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateAccountRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateAccountRequired {
        code: "your code",
        billing_info: BillingInfoCreate {
            cvv: Some("your cvv".to_owned()),
            address: Some(Address {
                street1: Some("your street 1".to_owned()),
                street2: Some("your street 2".to_owned()),
                region: Some("your region".to_owned()),
                phone: Some("your phone".to_owned()),
                city: Some("your city".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                country: Some("your country".to_owned()),
            }),
            company: Some("your company".to_owned()),
            amazon_billing_agreement_id: Some(
                "your amazon billing agreement id".to_owned(),
            ),
            token_id: Some("your token id".to_owned()),
            ip_address: Some("your ip address".to_owned()),
            type_: Some("your type".to_owned()),
            three_d_secure_action_result_token_id: Some(
                "your three d secure action result token id".to_owned(),
            ),
            last_name: Some("your last name".to_owned()),
            paypal_billing_agreement_id: Some(
                "your paypal billing agreement id".to_owned(),
            ),
            fraud_session_id: Some("your fraud session id".to_owned()),
            routing_number: Some("your routing number".to_owned()),
            account_type: Some("your account type".to_owned()),
            vat_number: Some("your vat number".to_owned()),
            first_name: Some("your first name".to_owned()),
            year: Some("your year".to_owned()),
            gateway_token: Some("your gateway token".to_owned()),
            name_on_account: Some("your name on account".to_owned()),
            number: Some("your number".to_owned()),
            account_number: Some("your account number".to_owned()),
            tax_identifier: Some("your tax identifier".to_owned()),
            primary_payment_method: Some(true),
            external_hpp_type: Some("your external hpp type".to_owned()),
            sort_code: Some("your sort code".to_owned()),
            online_banking_payment_type: Some(
                "your online banking payment type".to_owned(),
            ),
            month: Some("your month".to_owned()),
            backup_payment_method: Some(true),
            gateway_code: Some("your gateway code".to_owned()),
            card_type: Some("your card type".to_owned()),
            transaction_type: Some("your transaction type".to_owned()),
            tax_identifier_type: Some("your tax identifier type".to_owned()),
            iban: Some("your iban".to_owned()),
        },
        last_name: "your last name",
        custom_fields: CustomFields(
            vec![
                CustomField { value : "your value".to_owned(), name : "your name"
                .to_owned() }
            ],
        ),
        parent_account_id: "your parent account id",
        dunning_campaign_id: "your dunning campaign id",
        address: Address {
            street1: Some("your street 1".to_owned()),
            street2: Some("your street 2".to_owned()),
            region: Some("your region".to_owned()),
            phone: Some("your phone".to_owned()),
            city: Some("your city".to_owned()),
            postal_code: Some("your postal code".to_owned()),
            country: Some("your country".to_owned()),
        },
        preferred_locale: "your preferred locale",
        first_name: "your first name",
        email: "your email",
        cc_emails: "your cc emails",
        shipping_addresses: vec![
            ShippingAddressCreate { last_name : "your last name".to_owned(), nickname :
            Some("your nickname".to_owned()), city : "your city".to_owned(), phone :
            Some("your phone".to_owned()), company : Some("your company".to_owned()),
            email : Some("your email".to_owned()), street2 : Some("your street 2"
            .to_owned()), first_name : "your first name".to_owned(), region :
            Some("your region".to_owned()), vat_number : Some("your vat number"
            .to_owned()), street1 : "your street 1".to_owned(), postal_code :
            "your postal code".to_owned(), country : "your country".to_owned() }
        ],
        bill_to: "your bill to",
        company: "your company",
        username: "your username",
        transaction_type: "your transaction type",
        acquisition: AccountAcquisitionUpdate {
            subchannel: Some("your subchannel".to_owned()),
            cost: Some(::serde_json::json!({})),
            channel: Some("your channel".to_owned()),
            campaign: Some("your campaign".to_owned()),
        },
        tax_exempt: true,
        invoice_template_id: "your invoice template id",
        vat_number: "your vat number",
        exemption_certificate: "your exemption certificate",
        parent_account_code: "your parent account code",
    };
    let response = client.create_account(args).send().await.unwrap();
    println!("{:#?}", response);
}
