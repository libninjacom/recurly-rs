use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct VerifyBillingInfoRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub account_id: String,
    pub gateway_code: Option<String>,
}
impl<'a> VerifyBillingInfoRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Transaction> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/accounts/{account_id}/billing_info/verify", account_id = self
                    .account_id
                ),
            );
        if let Some(ref unwrapped) = self.gateway_code {
            r = r.push_json(json!({ "gateway_code" : unwrapped }));
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
    pub fn gateway_code(mut self, gateway_code: &str) -> Self {
        self.gateway_code = Some(gateway_code.to_owned());
        self
    }
}
