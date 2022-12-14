#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateAccountRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateAccountRequired {
        email: "your email",
        preferred_locale: "your preferred locale",
        username: "your username",
        invoice_template_id: "your invoice template id",
        transaction_type: "your transaction type",
        billing_info: BillingInfoCreate {
            vat_number: Some("your vat number".to_owned()),
            three_d_secure_action_result_token_id: Some(
                "your three d secure action result token id".to_owned(),
            ),
            card_type: Some("your card type".to_owned()),
            fraud_session_id: Some("your fraud session id".to_owned()),
            primary_payment_method: Some(true),
            online_banking_payment_type: Some(
                "your online banking payment type".to_owned(),
            ),
            last_name: Some("your last name".to_owned()),
            account_type: Some("your account type".to_owned()),
            name_on_account: Some("your name on account".to_owned()),
            gateway_token: Some("your gateway token".to_owned()),
            amazon_billing_agreement_id: Some(
                "your amazon billing agreement id".to_owned(),
            ),
            tax_identifier_type: Some("your tax identifier type".to_owned()),
            number: Some("your number".to_owned()),
            address: Some(Address {
                city: Some("your city".to_owned()),
                region: Some("your region".to_owned()),
                phone: Some("your phone".to_owned()),
                street2: Some("your street 2".to_owned()),
                postal_code: Some("your postal code".to_owned()),
                country: Some("your country".to_owned()),
                street1: Some("your street 1".to_owned()),
            }),
            ip_address: Some("your ip address".to_owned()),
            tax_identifier: Some("your tax identifier".to_owned()),
            backup_payment_method: Some(true),
            company: Some("your company".to_owned()),
            token_id: Some("your token id".to_owned()),
            gateway_code: Some("your gateway code".to_owned()),
            paypal_billing_agreement_id: Some(
                "your paypal billing agreement id".to_owned(),
            ),
            account_number: Some("your account number".to_owned()),
            first_name: Some("your first name".to_owned()),
            sort_code: Some("your sort code".to_owned()),
            cvv: Some("your cvv".to_owned()),
            month: Some("your month".to_owned()),
            iban: Some("your iban".to_owned()),
            routing_number: Some("your routing number".to_owned()),
            type_: Some("your type".to_owned()),
            external_hpp_type: Some("your external hpp type".to_owned()),
            transaction_type: Some("your transaction type".to_owned()),
            year: Some("your year".to_owned()),
        },
        code: "your code",
        exemption_certificate: "your exemption certificate",
        bill_to: "your bill to",
        address: Address {
            city: Some("your city".to_owned()),
            region: Some("your region".to_owned()),
            phone: Some("your phone".to_owned()),
            street2: Some("your street 2".to_owned()),
            postal_code: Some("your postal code".to_owned()),
            country: Some("your country".to_owned()),
            street1: Some("your street 1".to_owned()),
        },
        company: "your company",
        shipping_addresses: vec![
            ShippingAddressCreate { company : Some("your company".to_owned()), region :
            Some("your region".to_owned()), postal_code : "your postal code".to_owned(),
            country : "your country".to_owned(), street2 : Some("your street 2"
            .to_owned()), email : Some("your email".to_owned()), nickname :
            Some("your nickname".to_owned()), vat_number : Some("your vat number"
            .to_owned()), city : "your city".to_owned(), last_name : "your last name"
            .to_owned(), street1 : "your street 1".to_owned(), phone : Some("your phone"
            .to_owned()), first_name : "your first name".to_owned() }
        ],
        acquisition: AccountAcquisitionUpdate {
            cost: Some(::serde_json::json!({})),
            channel: Some("your channel".to_owned()),
            campaign: Some("your campaign".to_owned()),
            subchannel: Some("your subchannel".to_owned()),
        },
        parent_account_code: "your parent account code",
        tax_exempt: true,
        first_name: "your first name",
        cc_emails: "your cc emails",
        parent_account_id: "your parent account id",
        vat_number: "your vat number",
        last_name: "your last name",
        dunning_campaign_id: "your dunning campaign id",
        custom_fields: CustomFields(
            vec![
                CustomField { name : "your name".to_owned(), value : "your value"
                .to_owned() }
            ],
        ),
    };
    let response = client.create_account(args).send().await.unwrap();
    println!("{:#?}", response);
}
