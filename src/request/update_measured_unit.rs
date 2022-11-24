use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateMeasuredUnitRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub measured_unit_id: String,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
}
impl<'a> UpdateMeasuredUnitRequest<'a> {
    pub async fn send(self) -> anyhow::Result<MeasuredUnit> {
        let mut r = self
            .client
            .client
            .put(
                &format!(
                    "/measured_units/{measured_unit_id}", measured_unit_id = self
                    .measured_unit_id
                ),
            );
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.display_name {
            r = r.push_json(json!({ "display_name" : unwrapped }));
        }
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
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn display_name(mut self, display_name: &str) -> Self {
        self.display_name = Some(display_name.to_owned());
        self
    }
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());
        self
    }
}
