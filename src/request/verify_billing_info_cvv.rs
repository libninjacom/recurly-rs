use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct VerifyBillingInfoCvvRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub account_id: String,
    pub verification_value: Option<String>,
}
impl<'a> VerifyBillingInfoCvvRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Transaction> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/accounts/{account_id}/billing_info/verify_cvv", account_id = self
                    .account_id
                ),
            );
        if let Some(ref unwrapped) = self.verification_value {
            r = r.push_json(json!({ "verification_value" : unwrapped }));
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
    pub fn verification_value(mut self, verification_value: &str) -> Self {
        self.verification_value = Some(verification_value.to_owned());
        self
    }
}
