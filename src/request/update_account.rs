use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateAccountRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub account_id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub preferred_locale: Option<String>,
    pub cc_emails: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company: Option<String>,
    pub vat_number: Option<String>,
    pub tax_exempt: Option<bool>,
    pub exemption_certificate: Option<String>,
    pub parent_account_code: Option<String>,
    pub parent_account_id: Option<String>,
    pub bill_to: Option<String>,
    pub transaction_type: Option<String>,
    pub dunning_campaign_id: Option<String>,
    pub invoice_template_id: Option<String>,
    pub address: Option<Address>,
    pub billing_info: Option<BillingInfoCreate>,
    pub custom_fields: Option<CustomFields>,
}
impl<'a> UpdateAccountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Account> {
        let mut r = self
            .client
            .client
            .put(&format!("/accounts/{account_id}", account_id = self.account_id));
        if let Some(ref unwrapped) = self.username {
            r = r.push_json(json!({ "username" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.email {
            r = r.push_json(json!({ "email" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.preferred_locale {
            r = r.push_json(json!({ "preferred_locale" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.cc_emails {
            r = r.push_json(json!({ "cc_emails" : unwrapped }));
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
        if let Some(ref unwrapped) = self.vat_number {
            r = r.push_json(json!({ "vat_number" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tax_exempt {
            r = r.push_json(json!({ "tax_exempt" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.exemption_certificate {
            r = r.push_json(json!({ "exemption_certificate" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.parent_account_code {
            r = r.push_json(json!({ "parent_account_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.parent_account_id {
            r = r.push_json(json!({ "parent_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.bill_to {
            r = r.push_json(json!({ "bill_to" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.transaction_type {
            r = r.push_json(json!({ "transaction_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.dunning_campaign_id {
            r = r.push_json(json!({ "dunning_campaign_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.invoice_template_id {
            r = r.push_json(json!({ "invoice_template_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.address {
            r = r.push_json(json!({ "address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.billing_info {
            r = r.push_json(json!({ "billing_info" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.custom_fields {
            r = r.push_json(json!({ "custom_fields" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_owned());
        self
    }
    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_owned());
        self
    }
    pub fn preferred_locale(mut self, preferred_locale: &str) -> Self {
        self.preferred_locale = Some(preferred_locale.to_owned());
        self
    }
    pub fn cc_emails(mut self, cc_emails: &str) -> Self {
        self.cc_emails = Some(cc_emails.to_owned());
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
    pub fn vat_number(mut self, vat_number: &str) -> Self {
        self.vat_number = Some(vat_number.to_owned());
        self
    }
    pub fn tax_exempt(mut self, tax_exempt: bool) -> Self {
        self.tax_exempt = Some(tax_exempt);
        self
    }
    pub fn exemption_certificate(mut self, exemption_certificate: &str) -> Self {
        self.exemption_certificate = Some(exemption_certificate.to_owned());
        self
    }
    pub fn parent_account_code(mut self, parent_account_code: &str) -> Self {
        self.parent_account_code = Some(parent_account_code.to_owned());
        self
    }
    pub fn parent_account_id(mut self, parent_account_id: &str) -> Self {
        self.parent_account_id = Some(parent_account_id.to_owned());
        self
    }
    pub fn bill_to(mut self, bill_to: &str) -> Self {
        self.bill_to = Some(bill_to.to_owned());
        self
    }
    pub fn transaction_type(mut self, transaction_type: &str) -> Self {
        self.transaction_type = Some(transaction_type.to_owned());
        self
    }
    pub fn dunning_campaign_id(mut self, dunning_campaign_id: &str) -> Self {
        self.dunning_campaign_id = Some(dunning_campaign_id.to_owned());
        self
    }
    pub fn invoice_template_id(mut self, invoice_template_id: &str) -> Self {
        self.invoice_template_id = Some(invoice_template_id.to_owned());
        self
    }
    pub fn address(mut self, address: Address) -> Self {
        self.address = Some(address);
        self
    }
    pub fn billing_info(mut self, billing_info: BillingInfoCreate) -> Self {
        self.billing_info = Some(billing_info);
        self
    }
    pub fn custom_fields(mut self, custom_fields: CustomFields) -> Self {
        self.custom_fields = Some(custom_fields);
        self
    }
}
