use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListEntitlementsRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub account_id: String,
    pub state: Option<String>,
}
impl<'a> ListEntitlementsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Entitlements> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/accounts/{account_id}/entitlements", account_id = self.account_id
                ),
            );
        if let Some(ref unwrapped) = self.state {
            r = r.push_query("state", &unwrapped.to_string());
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
    pub fn state(mut self, state: &str) -> Self {
        self.state = Some(state.to_owned());
        self
    }
}
