use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RemoveSubscriptionChangeRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub subscription_id: String,
}
impl<'a> RemoveSubscriptionChangeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/subscriptions/{subscription_id}/change", subscription_id = self
                    .subscription_id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => Ok(()),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
