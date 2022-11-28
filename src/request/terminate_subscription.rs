use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TerminateSubscriptionRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub subscription_id: String,
    pub refund: Option<String>,
    pub charge: Option<bool>,
}
impl<'a> TerminateSubscriptionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Subscription> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!(
                    "/subscriptions/{subscription_id}", subscription_id = self
                    .subscription_id
                ),
            );
        if let Some(ref unwrapped) = self.refund {
            r = r.push_query("refund", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.charge {
            r = r.push_query("charge", &unwrapped.to_string());
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
    pub fn refund(mut self, refund: &str) -> Self {
        self.refund = Some(refund.to_owned());
        self
    }
    pub fn charge(mut self, charge: bool) -> Self {
        self.charge = Some(charge);
        self
    }
}
