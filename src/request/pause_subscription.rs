use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PauseSubscriptionRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub subscription_id: String,
    pub remaining_pause_cycles: i64,
}
impl<'a> PauseSubscriptionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Subscription> {
        let mut r = self
            .http_client
            .client
            .put(
                &format!(
                    "/subscriptions/{subscription_id}/pause", subscription_id = self
                    .subscription_id
                ),
            );
        r = r
            .push_json(
                json!({ "remaining_pause_cycles" : self.remaining_pause_cycles }),
            );
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
