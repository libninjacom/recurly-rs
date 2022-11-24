use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetExportFilesRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub export_date: String,
}
impl<'a> GetExportFilesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExportFiles> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/export_dates/{export_date}/export_files", export_date = self
                    .export_date
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
