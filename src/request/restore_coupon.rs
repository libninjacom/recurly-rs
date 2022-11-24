use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RestoreCouponRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub coupon_id: String,
    pub name: Option<String>,
    pub max_redemptions: Option<i64>,
    pub max_redemptions_per_account: Option<i64>,
    pub hosted_description: Option<String>,
    pub invoice_description: Option<String>,
    pub redeem_by_date: Option<String>,
}
impl<'a> RestoreCouponRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Coupon> {
        let mut r = self
            .client
            .client
            .put(&format!("/coupons/{coupon_id}/restore", coupon_id = self.coupon_id));
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.max_redemptions {
            r = r.push_json(json!({ "max_redemptions" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.max_redemptions_per_account {
            r = r.push_json(json!({ "max_redemptions_per_account" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.hosted_description {
            r = r.push_json(json!({ "hosted_description" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.invoice_description {
            r = r.push_json(json!({ "invoice_description" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.redeem_by_date {
            r = r.push_json(json!({ "redeem_by_date" : unwrapped }));
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
    pub fn max_redemptions(mut self, max_redemptions: i64) -> Self {
        self.max_redemptions = Some(max_redemptions);
        self
    }
    pub fn max_redemptions_per_account(
        mut self,
        max_redemptions_per_account: i64,
    ) -> Self {
        self.max_redemptions_per_account = Some(max_redemptions_per_account);
        self
    }
    pub fn hosted_description(mut self, hosted_description: &str) -> Self {
        self.hosted_description = Some(hosted_description.to_owned());
        self
    }
    pub fn invoice_description(mut self, invoice_description: &str) -> Self {
        self.invoice_description = Some(invoice_description.to_owned());
        self
    }
    pub fn redeem_by_date(mut self, redeem_by_date: &str) -> Self {
        self.redeem_by_date = Some(redeem_by_date.to_owned());
        self
    }
}
