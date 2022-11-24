use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RemovePlanRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub plan_id: String,
}
impl<'a> RemovePlanRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Plan> {
        let mut r = self
            .client
            .client
            .delete(&format!("/plans/{plan_id}", plan_id = self.plan_id));
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
