use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateBillingInfoRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub account_id: String,
    pub token_id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company: Option<String>,
    pub address: Option<Address>,
    pub number: Option<String>,
    pub month: Option<String>,
    pub year: Option<String>,
    pub cvv: Option<String>,
    pub vat_number: Option<String>,
    pub ip_address: Option<String>,
    pub gateway_token: Option<String>,
    pub gateway_code: Option<String>,
    pub amazon_billing_agreement_id: Option<String>,
    pub paypal_billing_agreement_id: Option<String>,
    pub fraud_session_id: Option<String>,
    pub transaction_type: Option<String>,
    pub three_d_secure_action_result_token_id: Option<String>,
    pub iban: Option<String>,
    pub name_on_account: Option<String>,
    pub account_number: Option<String>,
    pub routing_number: Option<String>,
    pub sort_code: Option<String>,
    pub type_: Option<String>,
    pub account_type: Option<String>,
    pub tax_identifier: Option<String>,
    pub tax_identifier_type: Option<String>,
    pub primary_payment_method: Option<bool>,
    pub backup_payment_method: Option<bool>,
    pub external_hpp_type: Option<String>,
    pub online_banking_payment_type: Option<String>,
    pub card_type: Option<String>,
}
impl<'a> CreateBillingInfoRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BillingInfo> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/accounts/{account_id}/billing_infos", account_id = self.account_id
                ),
            );
        if let Some(ref unwrapped) = self.token_id {
            r = r.push_json(json!({ "token_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.first_name {
            r = r.push_json(json!({ "first_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.last_name {
            r = r.push_json(json!({ "last_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.company {
            r = r.push_json(json!({ "company" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.address {
            r = r.push_json(json!({ "address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.number {
            r = r.push_json(json!({ "number" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.month {
            r = r.push_json(json!({ "month" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.year {
            r = r.push_json(json!({ "year" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.cvv {
            r = r.push_json(json!({ "cvv" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.vat_number {
            r = r.push_json(json!({ "vat_number" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.ip_address {
            r = r.push_json(json!({ "ip_address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.gateway_token {
            r = r.push_json(json!({ "gateway_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.gateway_code {
            r = r.push_json(json!({ "gateway_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.amazon_billing_agreement_id {
            r = r.push_json(json!({ "amazon_billing_agreement_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.paypal_billing_agreement_id {
            r = r.push_json(json!({ "paypal_billing_agreement_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.fraud_session_id {
            r = r.push_json(json!({ "fraud_session_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.transaction_type {
            r = r.push_json(json!({ "transaction_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.three_d_secure_action_result_token_id {
            r = r
                .push_json(
                    json!({ "three_d_secure_action_result_token_id" : unwrapped }),
                );
        }
        if let Some(ref unwrapped) = self.iban {
            r = r.push_json(json!({ "iban" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.name_on_account {
            r = r.push_json(json!({ "name_on_account" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.account_number {
            r = r.push_json(json!({ "account_number" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.routing_number {
            r = r.push_json(json!({ "routing_number" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.sort_code {
            r = r.push_json(json!({ "sort_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.push_json(json!({ "type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.account_type {
            r = r.push_json(json!({ "account_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tax_identifier {
            r = r.push_json(json!({ "tax_identifier" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tax_identifier_type {
            r = r.push_json(json!({ "tax_identifier_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.primary_payment_method {
            r = r.push_json(json!({ "primary_payment_method" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.backup_payment_method {
            r = r.push_json(json!({ "backup_payment_method" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.external_hpp_type {
            r = r.push_json(json!({ "external_hpp_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.online_banking_payment_type {
            r = r.push_json(json!({ "online_banking_payment_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.card_type {
            r = r.push_json(json!({ "card_type" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn token_id(mut self, token_id: &str) -> Self {
        self.token_id = Some(token_id.to_owned());
        self
    }
    pub fn first_name(mut self, first_name: &str) -> Self {
        self.first_name = Some(first_name.to_owned());
        self
    }
    pub fn last_name(mut self, last_name: &str) -> Self {
        self.last_name = Some(last_name.to_owned());
        self
    }
    pub fn company(mut self, company: &str) -> Self {
        self.company = Some(company.to_owned());
        self
    }
    pub fn address(mut self, address: Address) -> Self {
        self.address = Some(address);
        self
    }
    pub fn number(mut self, number: &str) -> Self {
        self.number = Some(number.to_owned());
        self
    }
    pub fn month(mut self, month: &str) -> Self {
        self.month = Some(month.to_owned());
        self
    }
    pub fn year(mut self, year: &str) -> Self {
        self.year = Some(year.to_owned());
        self
    }
    pub fn cvv(mut self, cvv: &str) -> Self {
        self.cvv = Some(cvv.to_owned());
        self
    }
    pub fn vat_number(mut self, vat_number: &str) -> Self {
        self.vat_number = Some(vat_number.to_owned());
        self
    }
    pub fn ip_address(mut self, ip_address: &str) -> Self {
        self.ip_address = Some(ip_address.to_owned());
        self
    }
    pub fn gateway_token(mut self, gateway_token: &str) -> Self {
        self.gateway_token = Some(gateway_token.to_owned());
        self
    }
    pub fn gateway_code(mut self, gateway_code: &str) -> Self {
        self.gateway_code = Some(gateway_code.to_owned());
        self
    }
    pub fn amazon_billing_agreement_id(
        mut self,
        amazon_billing_agreement_id: &str,
    ) -> Self {
        self.amazon_billing_agreement_id = Some(amazon_billing_agreement_id.to_owned());
        self
    }
    pub fn paypal_billing_agreement_id(
        mut self,
        paypal_billing_agreement_id: &str,
    ) -> Self {
        self.paypal_billing_agreement_id = Some(paypal_billing_agreement_id.to_owned());
        self
    }
    pub fn fraud_session_id(mut self, fraud_session_id: &str) -> Self {
        self.fraud_session_id = Some(fraud_session_id.to_owned());
        self
    }
    pub fn transaction_type(mut self, transaction_type: &str) -> Self {
        self.transaction_type = Some(transaction_type.to_owned());
        self
    }
    pub fn three_d_secure_action_result_token_id(
        mut self,
        three_d_secure_action_result_token_id: &str,
    ) -> Self {
        self
            .three_d_secure_action_result_token_id = Some(
            three_d_secure_action_result_token_id.to_owned(),
        );
        self
    }
    pub fn iban(mut self, iban: &str) -> Self {
        self.iban = Some(iban.to_owned());
        self
    }
    pub fn name_on_account(mut self, name_on_account: &str) -> Self {
        self.name_on_account = Some(name_on_account.to_owned());
        self
    }
    pub fn account_number(mut self, account_number: &str) -> Self {
        self.account_number = Some(account_number.to_owned());
        self
    }
    pub fn routing_number(mut self, routing_number: &str) -> Self {
        self.routing_number = Some(routing_number.to_owned());
        self
    }
    pub fn sort_code(mut self, sort_code: &str) -> Self {
        self.sort_code = Some(sort_code.to_owned());
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.type_ = Some(type_.to_owned());
        self
    }
    pub fn account_type(mut self, account_type: &str) -> Self {
        self.account_type = Some(account_type.to_owned());
        self
    }
    pub fn tax_identifier(mut self, tax_identifier: &str) -> Self {
        self.tax_identifier = Some(tax_identifier.to_owned());
        self
    }
    pub fn tax_identifier_type(mut self, tax_identifier_type: &str) -> Self {
        self.tax_identifier_type = Some(tax_identifier_type.to_owned());
        self
    }
    pub fn primary_payment_method(mut self, primary_payment_method: bool) -> Self {
        self.primary_payment_method = Some(primary_payment_method);
        self
    }
    pub fn backup_payment_method(mut self, backup_payment_method: bool) -> Self {
        self.backup_payment_method = Some(backup_payment_method);
        self
    }
    pub fn external_hpp_type(mut self, external_hpp_type: &str) -> Self {
        self.external_hpp_type = Some(external_hpp_type.to_owned());
        self
    }
    pub fn online_banking_payment_type(
        mut self,
        online_banking_payment_type: &str,
    ) -> Self {
        self.online_banking_payment_type = Some(online_banking_payment_type.to_owned());
        self
    }
    pub fn card_type(mut self, card_type: &str) -> Self {
        self.card_type = Some(card_type.to_owned());
        self
    }
}
