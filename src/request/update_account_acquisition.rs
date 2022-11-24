use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateAccountAcquisitionRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub account_id: String,
    pub cost: Option<serde_json::Value>,
    pub channel: Option<String>,
    pub subchannel: Option<String>,
    pub campaign: Option<String>,
}
impl<'a> UpdateAccountAcquisitionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AccountAcquisition> {
        let mut r = self
            .client
            .client
            .put(
                &format!(
                    "/accounts/{account_id}/acquisition", account_id = self.account_id
                ),
            );
        if let Some(ref unwrapped) = self.cost {
            r = r.push_json(json!({ "cost" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.channel {
            r = r.push_json(json!({ "channel" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.subchannel {
            r = r.push_json(json!({ "subchannel" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.campaign {
            r = r.push_json(json!({ "campaign" : unwrapped }));
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
    pub fn cost(mut self, cost: serde_json::Value) -> Self {
        self.cost = Some(cost);
        self
    }
    pub fn channel(mut self, channel: &str) -> Self {
        self.channel = Some(channel.to_owned());
        self
    }
    pub fn subchannel(mut self, subchannel: &str) -> Self {
        self.subchannel = Some(subchannel.to_owned());
        self
    }
    pub fn campaign(mut self, campaign: &str) -> Self {
        self.campaign = Some(campaign.to_owned());
        self
    }
}
