use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ConvertTrialRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub subscription_id: String,
}
impl<'a> ConvertTrialRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Subscription> {
        let mut r = self
            .client
            .client
            .put(
                &format!(
                    "/subscriptions/{subscription_id}/convert_trial", subscription_id =
                    self.subscription_id
                ),
            );
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
}
