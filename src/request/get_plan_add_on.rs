use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetPlanAddOnRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub plan_id: String,
    pub add_on_id: String,
}
impl<'a> GetPlanAddOnRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AddOn> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/plans/{plan_id}/add_ons/{add_on_id}", plan_id = self.plan_id,
                    add_on_id = self.add_on_id
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
