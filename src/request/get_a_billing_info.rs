use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetABillingInfoRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub account_id: String,
    pub billing_info_id: String,
}
impl<'a> GetABillingInfoRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BillingInfo> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/accounts/{account_id}/billing_infos/{billing_info_id}", account_id
                    = self.account_id, billing_info_id = self.billing_info_id
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
