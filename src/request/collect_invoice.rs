use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CollectInvoiceRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub invoice_id: String,
    pub three_d_secure_action_result_token_id: Option<String>,
    pub transaction_type: Option<String>,
    pub billing_info_id: Option<String>,
}
impl<'a> CollectInvoiceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Invoice> {
        let mut r = self
            .client
            .client
            .put(
                &format!("/invoices/{invoice_id}/collect", invoice_id = self.invoice_id),
            );
        if let Some(ref unwrapped) = self.three_d_secure_action_result_token_id {
            r = r
                .push_json(
                    json!({ "three_d_secure_action_result_token_id" : unwrapped }),
                );
        }
        if let Some(ref unwrapped) = self.transaction_type {
            r = r.push_json(json!({ "transaction_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.billing_info_id {
            r = r.push_json(json!({ "billing_info_id" : unwrapped }));
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
    pub fn transaction_type(mut self, transaction_type: &str) -> Self {
        self.transaction_type = Some(transaction_type.to_owned());
        self
    }
    pub fn billing_info_id(mut self, billing_info_id: &str) -> Self {
        self.billing_info_id = Some(billing_info_id.to_owned());
        self
    }
}
