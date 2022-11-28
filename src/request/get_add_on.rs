use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAddOnRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub add_on_id: String,
}
impl<'a> GetAddOnRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AddOn> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/add_ons/{add_on_id}", add_on_id = self.add_on_id));
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
