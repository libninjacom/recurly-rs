use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListAccountCreditPaymentsRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub account_id: String,
    pub limit: Option<i64>,
    pub order: Option<String>,
    pub sort: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}
impl<'a> ListAccountCreditPaymentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreditPaymentList> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/accounts/{account_id}/credit_payments", account_id = self
                    .account_id
                ),
            );
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
}
