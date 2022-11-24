use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateItemRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub item_id: String,
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub external_sku: Option<String>,
    pub accounting_code: Option<String>,
    pub revenue_schedule_type: Option<String>,
    pub avalara_transaction_type: Option<i64>,
    pub avalara_service_type: Option<i64>,
    pub tax_code: Option<String>,
    pub tax_exempt: Option<bool>,
    pub custom_fields: Option<CustomFields>,
    pub currencies: Option<Vec<Pricing>>,
}
impl<'a> UpdateItemRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Item> {
        let mut r = self
            .client
            .client
            .put(&format!("/items/{item_id}", item_id = self.item_id));
        if let Some(ref unwrapped) = self.code {
            r = r.push_json(json!({ "code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.description {
            r = r.push_json(json!({ "description" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.external_sku {
            r = r.push_json(json!({ "external_sku" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.accounting_code {
            r = r.push_json(json!({ "accounting_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.revenue_schedule_type {
            r = r.push_json(json!({ "revenue_schedule_type" : unwrapped }));
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
        if let Some(ref unwrapped) = self.custom_fields {
            r = r.push_json(json!({ "custom_fields" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.currencies {
            r = r.push_json(json!({ "currencies" : unwrapped }));
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
    pub fn external_sku(mut self, external_sku: &str) -> Self {
        self.external_sku = Some(external_sku.to_owned());
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
    pub fn custom_fields(mut self, custom_fields: CustomFields) -> Self {
        self.custom_fields = Some(custom_fields);
        self
    }
    pub fn currencies(mut self, currencies: Vec<Pricing>) -> Self {
        self.currencies = Some(currencies);
        self
    }
}
