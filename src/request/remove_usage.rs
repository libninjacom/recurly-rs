use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RemoveUsageRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub usage_id: String,
}
impl<'a> RemoveUsageRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .http_client
            .client
            .delete(&format!("/usage/{usage_id}", usage_id = self.usage_id));
        r = self.http_client.authenticate(r);
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
