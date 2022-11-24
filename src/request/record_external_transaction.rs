use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RecordExternalTransactionRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub invoice_id: String,
    pub payment_method: Option<String>,
    pub description: Option<String>,
    pub amount: Option<f64>,
    pub collected_at: Option<String>,
}
impl<'a> RecordExternalTransactionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Transaction> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/invoices/{invoice_id}/transactions", invoice_id = self.invoice_id
                ),
            );
        if let Some(ref unwrapped) = self.payment_method {
            r = r.push_json(json!({ "payment_method" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.description {
            r = r.push_json(json!({ "description" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.amount {
            r = r.push_json(json!({ "amount" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.collected_at {
            r = r.push_json(json!({ "collected_at" : unwrapped }));
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
    pub fn payment_method(mut self, payment_method: &str) -> Self {
        self.payment_method = Some(payment_method.to_owned());
        self
    }
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());
        self
    }
    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }
    pub fn collected_at(mut self, collected_at: &str) -> Self {
        self.collected_at = Some(collected_at.to_owned());
        self
    }
}
