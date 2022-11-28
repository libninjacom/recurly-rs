use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListAccountsRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub ids: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub order: Option<String>,
    pub sort: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
    pub email: Option<String>,
    pub subscriber: Option<bool>,
    pub past_due: Option<serde_json::Value>,
}
impl<'a> ListAccountsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AccountList> {
        let mut r = self.http_client.client.get("/accounts");
        if let Some(ref unwrapped) = self.ids {
            for item in unwrapped {
                r = r.push_query("ids[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.order {
            r = r.push_query("order", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sort {
            r = r.push_query("sort", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.begin_time {
            r = r.push_query("begin_time", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_time {
            r = r.push_query("end_time", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.email {
            r = r.push_query("email", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscriber {
            r = r.push_query("subscriber", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.past_due {
            r = r.push_query("past_due", &unwrapped.to_string());
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
    pub fn ids(mut self, ids: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.ids = Some(ids.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn order(mut self, order: &str) -> Self {
        self.order = Some(order.to_owned());
        self
    }
    pub fn sort(mut self, sort: &str) -> Self {
        self.sort = Some(sort.to_owned());
        self
    }
    pub fn begin_time(mut self, begin_time: &str) -> Self {
        self.begin_time = Some(begin_time.to_owned());
        self
    }
    pub fn end_time(mut self, end_time: &str) -> Self {
        self.end_time = Some(end_time.to_owned());
        self
    }
    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_owned());
        self
    }
    pub fn subscriber(mut self, subscriber: bool) -> Self {
        self.subscriber = Some(subscriber);
        self
    }
    pub fn past_due(mut self, past_due: serde_json::Value) -> Self {
        self.past_due = Some(past_due);
        self
    }
}
