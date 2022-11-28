use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetUniqueCouponCodeRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub unique_coupon_code_id: String,
}
impl<'a> GetUniqueCouponCodeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<UniqueCouponCode> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/unique_coupon_codes/{unique_coupon_code_id}", unique_coupon_code_id
                    = self.unique_coupon_code_id
                ),
            );
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
}
