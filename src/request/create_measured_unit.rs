use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateMeasuredUnitRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
}
impl<'a> CreateMeasuredUnitRequest<'a> {
    pub async fn send(self) -> anyhow::Result<MeasuredUnit> {
        let mut r = self.client.client.post("/measured_units");
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "display_name" : self.display_name }));
        if let Some(ref unwrapped) = self.description {
            r = r.push_json(json!({ "description" : unwrapped }));
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
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());
        self
    }
}
