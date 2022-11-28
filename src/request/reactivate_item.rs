use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ReactivateItemRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub item_id: String,
}
impl<'a> ReactivateItemRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Item> {
        let mut r = self
            .http_client
            .client
            .put(&format!("/items/{item_id}/reactivate", item_id = self.item_id));
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
