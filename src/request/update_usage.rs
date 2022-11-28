use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateUsageRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub usage_id: String,
    pub merchant_tag: Option<String>,
    pub amount: Option<f64>,
    pub recording_timestamp: Option<String>,
    pub usage_timestamp: Option<String>,
}
impl<'a> UpdateUsageRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Usage> {
        let mut r = self
            .http_client
            .client
            .put(&format!("/usage/{usage_id}", usage_id = self.usage_id));
        if let Some(ref unwrapped) = self.merchant_tag {
            r = r.push_json(json!({ "merchant_tag" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.amount {
            r = r.push_json(json!({ "amount" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.recording_timestamp {
            r = r.push_json(json!({ "recording_timestamp" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.usage_timestamp {
            r = r.push_json(json!({ "usage_timestamp" : unwrapped }));
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
    pub fn merchant_tag(mut self, merchant_tag: &str) -> Self {
        self.merchant_tag = Some(merchant_tag.to_owned());
        self
    }
    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }
    pub fn recording_timestamp(mut self, recording_timestamp: &str) -> Self {
        self.recording_timestamp = Some(recording_timestamp.to_owned());
        self
    }
    pub fn usage_timestamp(mut self, usage_timestamp: &str) -> Self {
        self.usage_timestamp = Some(usage_timestamp.to_owned());
        self
    }
}
