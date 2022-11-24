use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PutDunningCampaignBulkUpdateRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub dunning_campaign_id: String,
    pub plan_codes: Option<Vec<String>>,
    pub plan_ids: Option<Vec<String>>,
}
impl<'a> PutDunningCampaignBulkUpdateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DunningCampaignsBulkUpdateResponse> {
        let mut r = self
            .client
            .client
            .put(
                &format!(
                    "/dunning_campaigns/{dunning_campaign_id}/bulk_update",
                    dunning_campaign_id = self.dunning_campaign_id
                ),
            );
        if let Some(ref unwrapped) = self.plan_codes {
            r = r.push_json(json!({ "plan_codes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.plan_ids {
            r = r.push_json(json!({ "plan_ids" : unwrapped }));
        }
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
    pub fn plan_codes(
        mut self,
        plan_codes: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .plan_codes = Some(
            plan_codes.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn plan_ids(
        mut self,
        plan_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .plan_ids = Some(
            plan_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
