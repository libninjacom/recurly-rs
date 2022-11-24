use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateSubscriptionChangeRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub subscription_id: String,
    pub timeframe: Option<String>,
    pub plan_id: Option<String>,
    pub plan_code: Option<String>,
    pub unit_amount: Option<f64>,
    pub tax_inclusive: Option<bool>,
    pub quantity: Option<i64>,
    pub shipping: Option<SubscriptionChangeShippingCreate>,
    pub coupon_codes: Option<Vec<String>>,
    pub add_ons: Option<Vec<SubscriptionAddOnUpdate>>,
    pub collection_method: Option<String>,
    pub revenue_schedule_type: Option<String>,
    pub custom_fields: Option<CustomFields>,
    pub po_number: Option<String>,
    pub net_terms: Option<i64>,
    pub transaction_type: Option<String>,
    pub billing_info: Option<SubscriptionChangeBillingInfoCreate>,
    pub ramp_intervals: Option<Vec<SubscriptionRampInterval>>,
}
impl<'a> CreateSubscriptionChangeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SubscriptionChange> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/subscriptions/{subscription_id}/change", subscription_id = self
                    .subscription_id
                ),
            );
        if let Some(ref unwrapped) = self.timeframe {
            r = r.push_json(json!({ "timeframe" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.plan_id {
            r = r.push_json(json!({ "plan_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.plan_code {
            r = r.push_json(json!({ "plan_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.unit_amount {
            r = r.push_json(json!({ "unit_amount" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tax_inclusive {
            r = r.push_json(json!({ "tax_inclusive" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.quantity {
            r = r.push_json(json!({ "quantity" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.shipping {
            r = r.push_json(json!({ "shipping" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.coupon_codes {
            r = r.push_json(json!({ "coupon_codes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.add_ons {
            r = r.push_json(json!({ "add_ons" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.collection_method {
            r = r.push_json(json!({ "collection_method" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.revenue_schedule_type {
            r = r.push_json(json!({ "revenue_schedule_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.custom_fields {
            r = r.push_json(json!({ "custom_fields" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.po_number {
            r = r.push_json(json!({ "po_number" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.net_terms {
            r = r.push_json(json!({ "net_terms" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.transaction_type {
            r = r.push_json(json!({ "transaction_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.billing_info {
            r = r.push_json(json!({ "billing_info" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.ramp_intervals {
            r = r.push_json(json!({ "ramp_intervals" : unwrapped }));
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
    pub fn timeframe(mut self, timeframe: &str) -> Self {
        self.timeframe = Some(timeframe.to_owned());
        self
    }
    pub fn plan_id(mut self, plan_id: &str) -> Self {
        self.plan_id = Some(plan_id.to_owned());
        self
    }
    pub fn plan_code(mut self, plan_code: &str) -> Self {
        self.plan_code = Some(plan_code.to_owned());
        self
    }
    pub fn unit_amount(mut self, unit_amount: f64) -> Self {
        self.unit_amount = Some(unit_amount);
        self
    }
    pub fn tax_inclusive(mut self, tax_inclusive: bool) -> Self {
        self.tax_inclusive = Some(tax_inclusive);
        self
    }
    pub fn quantity(mut self, quantity: i64) -> Self {
        self.quantity = Some(quantity);
        self
    }
    pub fn shipping(mut self, shipping: SubscriptionChangeShippingCreate) -> Self {
        self.shipping = Some(shipping);
        self
    }
    pub fn coupon_codes(
        mut self,
        coupon_codes: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .coupon_codes = Some(
            coupon_codes.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn add_ons(mut self, add_ons: Vec<SubscriptionAddOnUpdate>) -> Self {
        self.add_ons = Some(add_ons);
        self
    }
    pub fn collection_method(mut self, collection_method: &str) -> Self {
        self.collection_method = Some(collection_method.to_owned());
        self
    }
    pub fn revenue_schedule_type(mut self, revenue_schedule_type: &str) -> Self {
        self.revenue_schedule_type = Some(revenue_schedule_type.to_owned());
        self
    }
    pub fn custom_fields(mut self, custom_fields: CustomFields) -> Self {
        self.custom_fields = Some(custom_fields);
        self
    }
    pub fn po_number(mut self, po_number: &str) -> Self {
        self.po_number = Some(po_number.to_owned());
        self
    }
    pub fn net_terms(mut self, net_terms: i64) -> Self {
        self.net_terms = Some(net_terms);
        self
    }
    pub fn transaction_type(mut self, transaction_type: &str) -> Self {
        self.transaction_type = Some(transaction_type.to_owned());
        self
    }
    pub fn billing_info(
        mut self,
        billing_info: SubscriptionChangeBillingInfoCreate,
    ) -> Self {
        self.billing_info = Some(billing_info);
        self
    }
    pub fn ramp_intervals(
        mut self,
        ramp_intervals: Vec<SubscriptionRampInterval>,
    ) -> Self {
        self.ramp_intervals = Some(ramp_intervals);
        self
    }
}
