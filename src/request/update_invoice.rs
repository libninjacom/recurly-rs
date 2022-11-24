use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateInvoiceRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub invoice_id: String,
    pub po_number: Option<String>,
    pub vat_reverse_charge_notes: Option<String>,
    pub terms_and_conditions: Option<String>,
    pub customer_notes: Option<String>,
    pub net_terms: Option<i64>,
    pub address: Option<InvoiceAddress>,
}
impl<'a> UpdateInvoiceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Invoice> {
        let mut r = self
            .client
            .client
            .put(&format!("/invoices/{invoice_id}", invoice_id = self.invoice_id));
        if let Some(ref unwrapped) = self.po_number {
            r = r.push_json(json!({ "po_number" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.vat_reverse_charge_notes {
            r = r.push_json(json!({ "vat_reverse_charge_notes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.terms_and_conditions {
            r = r.push_json(json!({ "terms_and_conditions" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.customer_notes {
            r = r.push_json(json!({ "customer_notes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.net_terms {
            r = r.push_json(json!({ "net_terms" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.address {
            r = r.push_json(json!({ "address" : unwrapped }));
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
    pub fn po_number(mut self, po_number: &str) -> Self {
        self.po_number = Some(po_number.to_owned());
        self
    }
    pub fn vat_reverse_charge_notes(mut self, vat_reverse_charge_notes: &str) -> Self {
        self.vat_reverse_charge_notes = Some(vat_reverse_charge_notes.to_owned());
        self
    }
    pub fn terms_and_conditions(mut self, terms_and_conditions: &str) -> Self {
        self.terms_and_conditions = Some(terms_and_conditions.to_owned());
        self
    }
    pub fn customer_notes(mut self, customer_notes: &str) -> Self {
        self.customer_notes = Some(customer_notes.to_owned());
        self
    }
    pub fn net_terms(mut self, net_terms: i64) -> Self {
        self.net_terms = Some(net_terms);
        self
    }
    pub fn address(mut self, address: InvoiceAddress) -> Self {
        self.address = Some(address);
        self
    }
}
