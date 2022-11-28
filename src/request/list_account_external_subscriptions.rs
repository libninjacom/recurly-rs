use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListAccountExternalSubscriptionsRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub sort: Option<String>,
    pub account_id: String,
}
impl<'a> ListAccountExternalSubscriptionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExternalSubscriptionList> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/accounts/{account_id}/external_subscriptions", account_id = self
                    .account_id
                ),
            );
        if let Some(ref unwrapped) = self.sort {
            r = r.push_query("sort", &unwrapped.to_string());
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
    pub fn sort(mut self, sort: &str) -> Self {
        self.sort = Some(sort.to_owned());
        self
    }
}
