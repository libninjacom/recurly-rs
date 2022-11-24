use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateCouponRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub name: String,
    pub max_redemptions: i64,
    pub max_redemptions_per_account: i64,
    pub hosted_description: String,
    pub invoice_description: String,
    pub redeem_by_date: String,
    pub code: String,
    pub discount_type: String,
    pub discount_percent: i64,
    pub free_trial_unit: String,
    pub free_trial_amount: i64,
    pub currencies: Vec<CouponPricing>,
    pub applies_to_non_plan_charges: bool,
    pub applies_to_all_plans: bool,
    pub applies_to_all_items: bool,
    pub plan_codes: Vec<String>,
    pub item_codes: Vec<String>,
    pub duration: String,
    pub temporal_amount: i64,
    pub temporal_unit: String,
    pub coupon_type: String,
    pub unique_code_template: String,
    pub redemption_resource: String,
}
impl<'a> CreateCouponRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Coupon> {
        let mut r = self.client.client.post("/coupons");
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "max_redemptions" : self.max_redemptions }));
        r = r
            .push_json(
                json!(
                    { "max_redemptions_per_account" : self.max_redemptions_per_account }
                ),
            );
        r = r.push_json(json!({ "hosted_description" : self.hosted_description }));
        r = r.push_json(json!({ "invoice_description" : self.invoice_description }));
        r = r.push_json(json!({ "redeem_by_date" : self.redeem_by_date }));
        r = r.push_json(json!({ "code" : self.code }));
        r = r.push_json(json!({ "discount_type" : self.discount_type }));
        r = r.push_json(json!({ "discount_percent" : self.discount_percent }));
        r = r.push_json(json!({ "free_trial_unit" : self.free_trial_unit }));
        r = r.push_json(json!({ "free_trial_amount" : self.free_trial_amount }));
        r = r.push_json(json!({ "currencies" : self.currencies }));
        r = r
            .push_json(
                json!(
                    { "applies_to_non_plan_charges" : self.applies_to_non_plan_charges }
                ),
            );
        r = r.push_json(json!({ "applies_to_all_plans" : self.applies_to_all_plans }));
        r = r.push_json(json!({ "applies_to_all_items" : self.applies_to_all_items }));
        r = r.push_json(json!({ "plan_codes" : self.plan_codes }));
        r = r.push_json(json!({ "item_codes" : self.item_codes }));
        r = r.push_json(json!({ "duration" : self.duration }));
        r = r.push_json(json!({ "temporal_amount" : self.temporal_amount }));
        r = r.push_json(json!({ "temporal_unit" : self.temporal_unit }));
        r = r.push_json(json!({ "coupon_type" : self.coupon_type }));
        r = r.push_json(json!({ "unique_code_template" : self.unique_code_template }));
        r = r.push_json(json!({ "redemption_resource" : self.redemption_resource }));
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
pub struct CreateCouponRequired<'a> {
    pub name: &'a str,
    pub max_redemptions: i64,
    pub max_redemptions_per_account: i64,
    pub hosted_description: &'a str,
    pub invoice_description: &'a str,
    pub redeem_by_date: &'a str,
    pub code: &'a str,
    pub discount_type: &'a str,
    pub discount_percent: i64,
    pub free_trial_unit: &'a str,
    pub free_trial_amount: i64,
    pub currencies: Vec<CouponPricing>,
    pub applies_to_non_plan_charges: bool,
    pub applies_to_all_plans: bool,
    pub applies_to_all_items: bool,
    pub plan_codes: &'a [&'a str],
    pub item_codes: &'a [&'a str],
    pub duration: &'a str,
    pub temporal_amount: i64,
    pub temporal_unit: &'a str,
    pub coupon_type: &'a str,
    pub unique_code_template: &'a str,
    pub redemption_resource: &'a str,
}
impl<'a> CreateCouponRequired<'a> {}
