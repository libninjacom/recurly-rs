use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RefundInvoiceRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub invoice_id: String,
    pub type_: String,
    pub amount: Option<f64>,
    pub line_items: Option<Vec<LineItemRefund>>,
    pub refund_method: Option<String>,
    pub credit_customer_notes: Option<String>,
    pub external_refund: Option<serde_json::Value>,
}
impl<'a> RefundInvoiceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Invoice> {
        let mut r = self
            .client
            .client
            .post(
                &format!("/invoices/{invoice_id}/refund", invoice_id = self.invoice_id),
            );
        r = r.push_json(json!({ "type" : self.type_ }));
        if let Some(ref unwrapped) = self.amount {
            r = r.push_json(json!({ "amount" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.line_items {
            r = r.push_json(json!({ "line_items" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.refund_method {
            r = r.push_json(json!({ "refund_method" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.credit_customer_notes {
            r = r.push_json(json!({ "credit_customer_notes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.external_refund {
            r = r.push_json(json!({ "external_refund" : unwrapped }));
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
    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }
    pub fn line_items(mut self, line_items: Vec<LineItemRefund>) -> Self {
        self.line_items = Some(line_items);
        self
    }
    pub fn refund_method(mut self, refund_method: &str) -> Self {
        self.refund_method = Some(refund_method.to_owned());
        self
    }
    pub fn credit_customer_notes(mut self, credit_customer_notes: &str) -> Self {
        self.credit_customer_notes = Some(credit_customer_notes.to_owned());
        self
    }
    pub fn external_refund(mut self, external_refund: serde_json::Value) -> Self {
        self.external_refund = Some(external_refund);
        self
    }
}
