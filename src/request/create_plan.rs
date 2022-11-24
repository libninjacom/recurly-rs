use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreatePlanRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub accounting_code: Option<String>,
    pub interval_unit: Option<String>,
    pub interval_length: Option<i64>,
    pub trial_unit: Option<String>,
    pub trial_length: Option<i64>,
    pub trial_requires_billing_info: Option<bool>,
    pub total_billing_cycles: Option<i64>,
    pub auto_renew: Option<bool>,
    pub pricing_model: Option<String>,
    pub ramp_intervals: Option<Vec<PlanRampInterval>>,
    pub custom_fields: Option<CustomFields>,
    pub revenue_schedule_type: Option<String>,
    pub setup_fee_revenue_schedule_type: Option<String>,
    pub setup_fee_accounting_code: Option<String>,
    pub avalara_transaction_type: Option<i64>,
    pub avalara_service_type: Option<i64>,
    pub tax_code: Option<String>,
    pub tax_exempt: Option<bool>,
    pub currencies: Vec<PlanPricing>,
    pub hosted_pages: Option<PlanHostedPages>,
    pub add_ons: Option<Vec<AddOnCreate>>,
    pub allow_any_item_on_subscriptions: Option<bool>,
    pub dunning_campaign_id: Option<String>,
}
impl<'a> CreatePlanRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Plan> {
        let mut r = self.client.client.post("/plans");
        r = r.push_json(json!({ "code" : self.code }));
        r = r.push_json(json!({ "name" : self.name }));
        if let Some(ref unwrapped) = self.description {
            r = r.push_json(json!({ "description" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.accounting_code {
            r = r.push_json(json!({ "accounting_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.interval_unit {
            r = r.push_json(json!({ "interval_unit" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.interval_length {
            r = r.push_json(json!({ "interval_length" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.trial_unit {
            r = r.push_json(json!({ "trial_unit" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.trial_length {
            r = r.push_json(json!({ "trial_length" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.trial_requires_billing_info {
            r = r.push_json(json!({ "trial_requires_billing_info" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.total_billing_cycles {
            r = r.push_json(json!({ "total_billing_cycles" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.auto_renew {
            r = r.push_json(json!({ "auto_renew" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.pricing_model {
            r = r.push_json(json!({ "pricing_model" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.ramp_intervals {
            r = r.push_json(json!({ "ramp_intervals" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.custom_fields {
            r = r.push_json(json!({ "custom_fields" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.revenue_schedule_type {
            r = r.push_json(json!({ "revenue_schedule_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.setup_fee_revenue_schedule_type {
            r = r.push_json(json!({ "setup_fee_revenue_schedule_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.setup_fee_accounting_code {
            r = r.push_json(json!({ "setup_fee_accounting_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.avalara_transaction_type {
            r = r.push_json(json!({ "avalara_transaction_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.avalara_service_type {
            r = r.push_json(json!({ "avalara_service_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tax_code {
            r = r.push_json(json!({ "tax_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tax_exempt {
            r = r.push_json(json!({ "tax_exempt" : unwrapped }));
        }
        r = r.push_json(json!({ "currencies" : self.currencies }));
        if let Some(ref unwrapped) = self.hosted_pages {
            r = r.push_json(json!({ "hosted_pages" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.add_ons {
            r = r.push_json(json!({ "add_ons" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.allow_any_item_on_subscriptions {
            r = r.push_json(json!({ "allow_any_item_on_subscriptions" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.dunning_campaign_id {
            r = r.push_json(json!({ "dunning_campaign_id" : unwrapped }));
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
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());
        self
    }
    pub fn accounting_code(mut self, accounting_code: &str) -> Self {
        self.accounting_code = Some(accounting_code.to_owned());
        self
    }
    pub fn interval_unit(mut self, interval_unit: &str) -> Self {
        self.interval_unit = Some(interval_unit.to_owned());
        self
    }
    pub fn interval_length(mut self, interval_length: i64) -> Self {
        self.interval_length = Some(interval_length);
        self
    }
    pub fn trial_unit(mut self, trial_unit: &str) -> Self {
        self.trial_unit = Some(trial_unit.to_owned());
        self
    }
    pub fn trial_length(mut self, trial_length: i64) -> Self {
        self.trial_length = Some(trial_length);
        self
    }
    pub fn trial_requires_billing_info(
        mut self,
        trial_requires_billing_info: bool,
    ) -> Self {
        self.trial_requires_billing_info = Some(trial_requires_billing_info);
        self
    }
    pub fn total_billing_cycles(mut self, total_billing_cycles: i64) -> Self {
        self.total_billing_cycles = Some(total_billing_cycles);
        self
    }
    pub fn auto_renew(mut self, auto_renew: bool) -> Self {
        self.auto_renew = Some(auto_renew);
        self
    }
    pub fn pricing_model(mut self, pricing_model: &str) -> Self {
        self.pricing_model = Some(pricing_model.to_owned());
        self
    }
    pub fn ramp_intervals(mut self, ramp_intervals: Vec<PlanRampInterval>) -> Self {
        self.ramp_intervals = Some(ramp_intervals);
        self
    }
    pub fn custom_fields(mut self, custom_fields: CustomFields) -> Self {
        self.custom_fields = Some(custom_fields);
        self
    }
    pub fn revenue_schedule_type(mut self, revenue_schedule_type: &str) -> Self {
        self.revenue_schedule_type = Some(revenue_schedule_type.to_owned());
        self
    }
    pub fn setup_fee_revenue_schedule_type(
        mut self,
        setup_fee_revenue_schedule_type: &str,
    ) -> Self {
        self
            .setup_fee_revenue_schedule_type = Some(
            setup_fee_revenue_schedule_type.to_owned(),
        );
        self
    }
    pub fn setup_fee_accounting_code(mut self, setup_fee_accounting_code: &str) -> Self {
        self.setup_fee_accounting_code = Some(setup_fee_accounting_code.to_owned());
        self
    }
    pub fn avalara_transaction_type(mut self, avalara_transaction_type: i64) -> Self {
        self.avalara_transaction_type = Some(avalara_transaction_type);
        self
    }
    pub fn avalara_service_type(mut self, avalara_service_type: i64) -> Self {
        self.avalara_service_type = Some(avalara_service_type);
        self
    }
    pub fn tax_code(mut self, tax_code: &str) -> Self {
        self.tax_code = Some(tax_code.to_owned());
        self
    }
    pub fn tax_exempt(mut self, tax_exempt: bool) -> Self {
        self.tax_exempt = Some(tax_exempt);
        self
    }
    pub fn hosted_pages(mut self, hosted_pages: PlanHostedPages) -> Self {
        self.hosted_pages = Some(hosted_pages);
        self
    }
    pub fn add_ons(mut self, add_ons: Vec<AddOnCreate>) -> Self {
        self.add_ons = Some(add_ons);
        self
    }
    pub fn allow_any_item_on_subscriptions(
        mut self,
        allow_any_item_on_subscriptions: bool,
    ) -> Self {
        self.allow_any_item_on_subscriptions = Some(allow_any_item_on_subscriptions);
        self
    }
    pub fn dunning_campaign_id(mut self, dunning_campaign_id: &str) -> Self {
        self.dunning_campaign_id = Some(dunning_campaign_id.to_owned());
        self
    }
}
