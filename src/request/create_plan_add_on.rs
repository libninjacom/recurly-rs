use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreatePlanAddOnRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub plan_id: String,
    pub item_code: Option<String>,
    pub item_id: Option<String>,
    pub code: String,
    pub name: String,
    pub add_on_type: Option<String>,
    pub usage_type: Option<String>,
    pub usage_calculation_type: Option<String>,
    pub usage_percentage: Option<f64>,
    pub measured_unit_id: Option<String>,
    pub measured_unit_name: Option<String>,
    pub accounting_code: Option<String>,
    pub revenue_schedule_type: Option<String>,
    pub display_quantity: Option<bool>,
    pub default_quantity: Option<i64>,
    pub optional: Option<bool>,
    pub avalara_transaction_type: Option<i64>,
    pub avalara_service_type: Option<i64>,
    pub tax_code: Option<String>,
    pub currencies: Option<Vec<AddOnPricing>>,
    pub tier_type: Option<String>,
    pub usage_timeframe: Option<String>,
    pub tiers: Option<Vec<Tier>>,
    pub percentage_tiers: Option<Vec<PercentageTiersByCurrency>>,
}
impl<'a> CreatePlanAddOnRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AddOn> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/plans/{plan_id}/add_ons", plan_id = self.plan_id));
        if let Some(ref unwrapped) = self.item_code {
            r = r.push_json(json!({ "item_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.item_id {
            r = r.push_json(json!({ "item_id" : unwrapped }));
        }
        r = r.push_json(json!({ "code" : self.code }));
        r = r.push_json(json!({ "name" : self.name }));
        if let Some(ref unwrapped) = self.add_on_type {
            r = r.push_json(json!({ "add_on_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.usage_type {
            r = r.push_json(json!({ "usage_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.usage_calculation_type {
            r = r.push_json(json!({ "usage_calculation_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.usage_percentage {
            r = r.push_json(json!({ "usage_percentage" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.measured_unit_id {
            r = r.push_json(json!({ "measured_unit_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.measured_unit_name {
            r = r.push_json(json!({ "measured_unit_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.accounting_code {
            r = r.push_json(json!({ "accounting_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.revenue_schedule_type {
            r = r.push_json(json!({ "revenue_schedule_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.display_quantity {
            r = r.push_json(json!({ "display_quantity" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.default_quantity {
            r = r.push_json(json!({ "default_quantity" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.optional {
            r = r.push_json(json!({ "optional" : unwrapped }));
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
        if let Some(ref unwrapped) = self.currencies {
            r = r.push_json(json!({ "currencies" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tier_type {
            r = r.push_json(json!({ "tier_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.usage_timeframe {
            r = r.push_json(json!({ "usage_timeframe" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tiers {
            r = r.push_json(json!({ "tiers" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.percentage_tiers {
            r = r.push_json(json!({ "percentage_tiers" : unwrapped }));
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
    pub fn item_code(mut self, item_code: &str) -> Self {
        self.item_code = Some(item_code.to_owned());
        self
    }
    pub fn item_id(mut self, item_id: &str) -> Self {
        self.item_id = Some(item_id.to_owned());
        self
    }
    pub fn add_on_type(mut self, add_on_type: &str) -> Self {
        self.add_on_type = Some(add_on_type.to_owned());
        self
    }
    pub fn usage_type(mut self, usage_type: &str) -> Self {
        self.usage_type = Some(usage_type.to_owned());
        self
    }
    pub fn usage_calculation_type(mut self, usage_calculation_type: &str) -> Self {
        self.usage_calculation_type = Some(usage_calculation_type.to_owned());
        self
    }
    pub fn usage_percentage(mut self, usage_percentage: f64) -> Self {
        self.usage_percentage = Some(usage_percentage);
        self
    }
    pub fn measured_unit_id(mut self, measured_unit_id: &str) -> Self {
        self.measured_unit_id = Some(measured_unit_id.to_owned());
        self
    }
    pub fn measured_unit_name(mut self, measured_unit_name: &str) -> Self {
        self.measured_unit_name = Some(measured_unit_name.to_owned());
        self
    }
    pub fn accounting_code(mut self, accounting_code: &str) -> Self {
        self.accounting_code = Some(accounting_code.to_owned());
        self
    }
    pub fn revenue_schedule_type(mut self, revenue_schedule_type: &str) -> Self {
        self.revenue_schedule_type = Some(revenue_schedule_type.to_owned());
        self
    }
    pub fn display_quantity(mut self, display_quantity: bool) -> Self {
        self.display_quantity = Some(display_quantity);
        self
    }
    pub fn default_quantity(mut self, default_quantity: i64) -> Self {
        self.default_quantity = Some(default_quantity);
        self
    }
    pub fn optional(mut self, optional: bool) -> Self {
        self.optional = Some(optional);
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
    pub fn currencies(mut self, currencies: Vec<AddOnPricing>) -> Self {
        self.currencies = Some(currencies);
        self
    }
    pub fn tier_type(mut self, tier_type: &str) -> Self {
        self.tier_type = Some(tier_type.to_owned());
        self
    }
    pub fn usage_timeframe(mut self, usage_timeframe: &str) -> Self {
        self.usage_timeframe = Some(usage_timeframe.to_owned());
        self
    }
    pub fn tiers(mut self, tiers: Vec<Tier>) -> Self {
        self.tiers = Some(tiers);
        self
    }
    pub fn percentage_tiers(
        mut self,
        percentage_tiers: Vec<PercentageTiersByCurrency>,
    ) -> Self {
        self.percentage_tiers = Some(percentage_tiers);
        self
    }
}
