use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateAccountRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub code: String,
    pub acquisition: AccountAcquisitionUpdate,
    pub shipping_addresses: Vec<ShippingAddressCreate>,
    pub username: String,
    pub email: String,
    pub preferred_locale: String,
    pub cc_emails: String,
    pub first_name: String,
    pub last_name: String,
    pub company: String,
    pub vat_number: String,
    pub tax_exempt: bool,
    pub exemption_certificate: String,
    pub parent_account_code: String,
    pub parent_account_id: String,
    pub bill_to: String,
    pub transaction_type: String,
    pub dunning_campaign_id: String,
    pub invoice_template_id: String,
    pub address: Address,
    pub billing_info: BillingInfoCreate,
    pub custom_fields: CustomFields,
}
impl<'a> CreateAccountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Account> {
        let mut r = self.http_client.client.post("/accounts");
        r = r.push_json(json!({ "code" : self.code }));
        r = r.push_json(json!({ "acquisition" : self.acquisition }));
        r = r.push_json(json!({ "shipping_addresses" : self.shipping_addresses }));
        r = r.push_json(json!({ "username" : self.username }));
        r = r.push_json(json!({ "email" : self.email }));
        r = r.push_json(json!({ "preferred_locale" : self.preferred_locale }));
        r = r.push_json(json!({ "cc_emails" : self.cc_emails }));
        r = r.push_json(json!({ "first_name" : self.first_name }));
        r = r.push_json(json!({ "last_name" : self.last_name }));
        r = r.push_json(json!({ "company" : self.company }));
        r = r.push_json(json!({ "vat_number" : self.vat_number }));
        r = r.push_json(json!({ "tax_exempt" : self.tax_exempt }));
        r = r.push_json(json!({ "exemption_certificate" : self.exemption_certificate }));
        r = r.push_json(json!({ "parent_account_code" : self.parent_account_code }));
        r = r.push_json(json!({ "parent_account_id" : self.parent_account_id }));
        r = r.push_json(json!({ "bill_to" : self.bill_to }));
        r = r.push_json(json!({ "transaction_type" : self.transaction_type }));
        r = r.push_json(json!({ "dunning_campaign_id" : self.dunning_campaign_id }));
        r = r.push_json(json!({ "invoice_template_id" : self.invoice_template_id }));
        r = r.push_json(json!({ "address" : self.address }));
        r = r.push_json(json!({ "billing_info" : self.billing_info }));
        r = r.push_json(json!({ "custom_fields" : self.custom_fields }));
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
}
pub struct CreateAccountRequired<'a> {
    pub code: &'a str,
    pub acquisition: AccountAcquisitionUpdate,
    pub shipping_addresses: Vec<ShippingAddressCreate>,
    pub username: &'a str,
    pub email: &'a str,
    pub preferred_locale: &'a str,
    pub cc_emails: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub company: &'a str,
    pub vat_number: &'a str,
    pub tax_exempt: bool,
    pub exemption_certificate: &'a str,
    pub parent_account_code: &'a str,
    pub parent_account_id: &'a str,
    pub bill_to: &'a str,
    pub transaction_type: &'a str,
    pub dunning_campaign_id: &'a str,
    pub invoice_template_id: &'a str,
    pub address: Address,
    pub billing_info: BillingInfoCreate,
    pub custom_fields: CustomFields,
}
impl<'a> CreateAccountRequired<'a> {}
