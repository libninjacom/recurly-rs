use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListAccountNotesRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub account_id: String,
    pub ids: Option<Vec<String>>,
}
impl<'a> ListAccountNotesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AccountNoteList> {
        let mut r = self
            .client
            .client
            .get(&format!("/accounts/{account_id}/notes", account_id = self.account_id));
        if let Some(ref unwrapped) = self.ids {
            for item in unwrapped {
                r = r.push_query("ids[]", &item.to_string());
            }
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
    pub fn ids(mut self, ids: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.ids = Some(ids.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
