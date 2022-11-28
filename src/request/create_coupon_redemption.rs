use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateCouponRedemptionRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub account_id: String,
    pub coupon_id: String,
    pub currency: Option<String>,
    pub subscription_id: Option<String>,
}
impl<'a> CreateCouponRedemptionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CouponRedemption> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/accounts/{account_id}/coupon_redemptions/active", account_id = self
                    .account_id
                ),
            );
        r = r.push_json(json!({ "coupon_id" : self.coupon_id }));
        if let Some(ref unwrapped) = self.currency {
            r = r.push_json(json!({ "currency" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.subscription_id {
            r = r.push_json(json!({ "subscription_id" : unwrapped }));
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
    pub fn currency(mut self, currency: &str) -> Self {
        self.currency = Some(currency.to_owned());
        self
    }
    pub fn subscription_id(mut self, subscription_id: &str) -> Self {
        self.subscription_id = Some(subscription_id.to_owned());
        self
    }
}
