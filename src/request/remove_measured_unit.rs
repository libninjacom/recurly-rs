use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RemoveMeasuredUnitRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub measured_unit_id: String,
}
impl<'a> RemoveMeasuredUnitRequest<'a> {
    pub async fn send(self) -> anyhow::Result<MeasuredUnit> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/measured_units/{measured_unit_id}", measured_unit_id = self
                    .measured_unit_id
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
