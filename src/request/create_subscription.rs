use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateSubscriptionRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub plan_code: String,
    pub plan_id: Option<String>,
    pub account: AccountCreate,
    pub billing_info_id: Option<String>,
    pub shipping: Option<SubscriptionShippingCreate>,
    pub collection_method: Option<String>,
    pub currency: String,
    pub unit_amount: Option<f64>,
    pub tax_inclusive: Option<bool>,
    pub quantity: Option<i64>,
    pub add_ons: Option<Vec<SubscriptionAddOnCreate>>,
    pub coupon_codes: Option<Vec<String>>,
    pub custom_fields: Option<CustomFields>,
    pub trial_ends_at: Option<String>,
    pub starts_at: Option<String>,
    pub next_bill_date: Option<String>,
    pub total_billing_cycles: Option<i64>,
    pub renewal_billing_cycles: Option<i64>,
    pub auto_renew: Option<bool>,
    pub ramp_intervals: Option<Vec<SubscriptionRampInterval>>,
    pub revenue_schedule_type: Option<String>,
    pub terms_and_conditions: Option<String>,
    pub customer_notes: Option<String>,
    pub credit_customer_notes: Option<String>,
    pub po_number: Option<String>,
    pub net_terms: Option<i64>,
    pub transaction_type: Option<String>,
}
impl<'a> CreateSubscriptionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Subscription> {
        let mut r = self.client.client.post("/subscriptions");
        r = r.push_json(json!({ "plan_code" : self.plan_code }));
        if let Some(ref unwrapped) = self.plan_id {
            r = r.push_json(json!({ "plan_id" : unwrapped }));
        }
        r = r.push_json(json!({ "account" : self.account }));
        if let Some(ref unwrapped) = self.billing_info_id {
            r = r.push_json(json!({ "billing_info_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.shipping {
            r = r.push_json(json!({ "shipping" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.collection_method {
            r = r.push_json(json!({ "collection_method" : unwrapped }));
        }
        r = r.push_json(json!({ "currency" : self.currency }));
        if let Some(ref unwrapped) = self.unit_amount {
            r = r.push_json(json!({ "unit_amount" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tax_inclusive {
            r = r.push_json(json!({ "tax_inclusive" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.quantity {
            r = r.push_json(json!({ "quantity" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.add_ons {
            r = r.push_json(json!({ "add_ons" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.coupon_codes {
            r = r.push_json(json!({ "coupon_codes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.custom_fields {
            r = r.push_json(json!({ "custom_fields" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.trial_ends_at {
            r = r.push_json(json!({ "trial_ends_at" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.starts_at {
            r = r.push_json(json!({ "starts_at" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.next_bill_date {
            r = r.push_json(json!({ "next_bill_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.total_billing_cycles {
            r = r.push_json(json!({ "total_billing_cycles" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.renewal_billing_cycles {
            r = r.push_json(json!({ "renewal_billing_cycles" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.auto_renew {
            r = r.push_json(json!({ "auto_renew" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.ramp_intervals {
            r = r.push_json(json!({ "ramp_intervals" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.revenue_schedule_type {
            r = r.push_json(json!({ "revenue_schedule_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.terms_and_conditions {
            r = r.push_json(json!({ "terms_and_conditions" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.customer_notes {
            r = r.push_json(json!({ "customer_notes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.credit_customer_notes {
            r = r.push_json(json!({ "credit_customer_notes" : unwrapped }));
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
    pub fn plan_id(mut self, plan_id: &str) -> Self {
        self.plan_id = Some(plan_id.to_owned());
        self
    }
    pub fn billing_info_id(mut self, billing_info_id: &str) -> Self {
        self.billing_info_id = Some(billing_info_id.to_owned());
        self
    }
    pub fn shipping(mut self, shipping: SubscriptionShippingCreate) -> Self {
        self.shipping = Some(shipping);
        self
    }
    pub fn collection_method(mut self, collection_method: &str) -> Self {
        self.collection_method = Some(collection_method.to_owned());
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
    pub fn add_ons(mut self, add_ons: Vec<SubscriptionAddOnCreate>) -> Self {
        self.add_ons = Some(add_ons);
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
    pub fn custom_fields(mut self, custom_fields: CustomFields) -> Self {
        self.custom_fields = Some(custom_fields);
        self
    }
    pub fn trial_ends_at(mut self, trial_ends_at: &str) -> Self {
        self.trial_ends_at = Some(trial_ends_at.to_owned());
        self
    }
    pub fn starts_at(mut self, starts_at: &str) -> Self {
        self.starts_at = Some(starts_at.to_owned());
        self
    }
    pub fn next_bill_date(mut self, next_bill_date: &str) -> Self {
        self.next_bill_date = Some(next_bill_date.to_owned());
        self
    }
    pub fn total_billing_cycles(mut self, total_billing_cycles: i64) -> Self {
        self.total_billing_cycles = Some(total_billing_cycles);
        self
    }
    pub fn renewal_billing_cycles(mut self, renewal_billing_cycles: i64) -> Self {
        self.renewal_billing_cycles = Some(renewal_billing_cycles);
        self
    }
    pub fn auto_renew(mut self, auto_renew: bool) -> Self {
        self.auto_renew = Some(auto_renew);
        self
    }
    pub fn ramp_intervals(
        mut self,
        ramp_intervals: Vec<SubscriptionRampInterval>,
    ) -> Self {
        self.ramp_intervals = Some(ramp_intervals);
        self
    }
    pub fn revenue_schedule_type(mut self, revenue_schedule_type: &str) -> Self {
        self.revenue_schedule_type = Some(revenue_schedule_type.to_owned());
        self
    }
    pub fn terms_and_conditions(mut self, terms_and_conditions: &str) -> Self {
        self.terms_and_conditions = Some(terms_and_conditions.to_owned());
        self
    }
    pub fn customer_notes(mut self, customer_notes: &str) -> Self {
        self.customer_notes = Some(customer_notes.to_owned());
        self
    }
    pub fn credit_customer_notes(mut self, credit_customer_notes: &str) -> Self {
        self.credit_customer_notes = Some(credit_customer_notes.to_owned());
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
}
