use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GenerateUniqueCouponCodesRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub coupon_id: String,
    pub number_of_unique_codes: Option<i64>,
}
impl<'a> GenerateUniqueCouponCodesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<UniqueCouponCodeParams> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/coupons/{coupon_id}/generate", coupon_id = self.coupon_id));
        if let Some(ref unwrapped) = self.number_of_unique_codes {
            r = r.push_json(json!({ "number_of_unique_codes" : unwrapped }));
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
    pub fn number_of_unique_codes(mut self, number_of_unique_codes: i64) -> Self {
        self.number_of_unique_codes = Some(number_of_unique_codes);
        self
    }
}
