use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateInvoiceRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub account_id: String,
    pub currency: String,
    pub collection_method: Option<String>,
    pub charge_customer_notes: Option<String>,
    pub credit_customer_notes: Option<String>,
    pub net_terms: Option<i64>,
    pub po_number: Option<String>,
    pub terms_and_conditions: Option<String>,
    pub vat_reverse_charge_notes: Option<String>,
}
impl<'a> CreateInvoiceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InvoiceCollection> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!("/accounts/{account_id}/invoices", account_id = self.account_id),
            );
        r = r.push_json(json!({ "currency" : self.currency }));
        if let Some(ref unwrapped) = self.collection_method {
            r = r.push_json(json!({ "collection_method" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.charge_customer_notes {
            r = r.push_json(json!({ "charge_customer_notes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.credit_customer_notes {
            r = r.push_json(json!({ "credit_customer_notes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.net_terms {
            r = r.push_json(json!({ "net_terms" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.po_number {
            r = r.push_json(json!({ "po_number" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.terms_and_conditions {
            r = r.push_json(json!({ "terms_and_conditions" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.vat_reverse_charge_notes {
            r = r.push_json(json!({ "vat_reverse_charge_notes" : unwrapped }));
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
    pub fn collection_method(mut self, collection_method: &str) -> Self {
        self.collection_method = Some(collection_method.to_owned());
        self
    }
    pub fn charge_customer_notes(mut self, charge_customer_notes: &str) -> Self {
        self.charge_customer_notes = Some(charge_customer_notes.to_owned());
        self
    }
    pub fn credit_customer_notes(mut self, credit_customer_notes: &str) -> Self {
        self.credit_customer_notes = Some(credit_customer_notes.to_owned());
        self
    }
    pub fn net_terms(mut self, net_terms: i64) -> Self {
        self.net_terms = Some(net_terms);
        self
    }
    pub fn po_number(mut self, po_number: &str) -> Self {
        self.po_number = Some(po_number.to_owned());
        self
    }
    pub fn terms_and_conditions(mut self, terms_and_conditions: &str) -> Self {
        self.terms_and_conditions = Some(terms_and_conditions.to_owned());
        self
    }
    pub fn vat_reverse_charge_notes(mut self, vat_reverse_charge_notes: &str) -> Self {
        self.vat_reverse_charge_notes = Some(vat_reverse_charge_notes.to_owned());
        self
    }
}
