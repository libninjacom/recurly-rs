use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdatePlanRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub plan_id: String,
    pub id: Option<String>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub accounting_code: Option<String>,
    pub trial_unit: Option<String>,
    pub trial_length: Option<i64>,
    pub trial_requires_billing_info: Option<bool>,
    pub total_billing_cycles: Option<i64>,
    pub auto_renew: Option<bool>,
    pub ramp_intervals: Option<Vec<PlanRampInterval>>,
    pub custom_fields: Option<CustomFields>,
    pub revenue_schedule_type: Option<String>,
    pub setup_fee_revenue_schedule_type: Option<String>,
    pub setup_fee_accounting_code: Option<String>,
    pub avalara_transaction_type: Option<i64>,
    pub avalara_service_type: Option<i64>,
    pub tax_code: Option<String>,
    pub tax_exempt: Option<bool>,
    pub currencies: Option<Vec<PlanPricing>>,
    pub hosted_pages: Option<PlanHostedPages>,
    pub allow_any_item_on_subscriptions: Option<bool>,
    pub dunning_campaign_id: Option<String>,
}
impl<'a> UpdatePlanRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Plan> {
        let mut r = self
            .http_client
            .client
            .put(&format!("/plans/{plan_id}", plan_id = self.plan_id));
        if let Some(ref unwrapped) = self.id {
            r = r.push_json(json!({ "id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.code {
            r = r.push_json(json!({ "code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.description {
            r = r.push_json(json!({ "description" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.accounting_code {
            r = r.push_json(json!({ "accounting_code" : unwrapped }));
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
        if let Some(ref unwrapped) = self.currencies {
            r = r.push_json(json!({ "currencies" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.hosted_pages {
            r = r.push_json(json!({ "hosted_pages" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.allow_any_item_on_subscriptions {
            r = r.push_json(json!({ "allow_any_item_on_subscriptions" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.dunning_campaign_id {
            r = r.push_json(json!({ "dunning_campaign_id" : unwrapped }));
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
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_owned());
        self
    }
    pub fn code(mut self, code: &str) -> Self {
        self.code = Some(code.to_owned());
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());
        self
    }
    pub fn accounting_code(mut self, accounting_code: &str) -> Self {
        self.accounting_code = Some(accounting_code.to_owned());
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
    pub fn currencies(mut self, currencies: Vec<PlanPricing>) -> Self {
        self.currencies = Some(currencies);
        self
    }
    pub fn hosted_pages(mut self, hosted_pages: PlanHostedPages) -> Self {
        self.hosted_pages = Some(hosted_pages);
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
