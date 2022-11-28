use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateLineItemRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub account_id: String,
    pub currency: String,
    pub unit_amount: f64,
    pub tax_inclusive: Option<bool>,
    pub quantity: Option<i64>,
    pub description: Option<String>,
    pub item_code: Option<String>,
    pub item_id: Option<String>,
    pub revenue_schedule_type: Option<String>,
    pub type_: String,
    pub credit_reason_code: Option<String>,
    pub accounting_code: Option<String>,
    pub tax_exempt: Option<bool>,
    pub avalara_transaction_type: Option<i64>,
    pub avalara_service_type: Option<i64>,
    pub tax_code: Option<String>,
    pub product_code: Option<String>,
    pub origin: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}
impl<'a> CreateLineItemRequest<'a> {
    pub async fn send(self) -> anyhow::Result<LineItem> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/accounts/{account_id}/line_items", account_id = self.account_id
                ),
            );
        r = r.push_json(json!({ "currency" : self.currency }));
        r = r.push_json(json!({ "unit_amount" : self.unit_amount }));
        if let Some(ref unwrapped) = self.tax_inclusive {
            r = r.push_json(json!({ "tax_inclusive" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.quantity {
            r = r.push_json(json!({ "quantity" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.description {
            r = r.push_json(json!({ "description" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.item_code {
            r = r.push_json(json!({ "item_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.item_id {
            r = r.push_json(json!({ "item_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.revenue_schedule_type {
            r = r.push_json(json!({ "revenue_schedule_type" : unwrapped }));
        }
        r = r.push_json(json!({ "type" : self.type_ }));
        if let Some(ref unwrapped) = self.credit_reason_code {
            r = r.push_json(json!({ "credit_reason_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.accounting_code {
            r = r.push_json(json!({ "accounting_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tax_exempt {
            r = r.push_json(json!({ "tax_exempt" : unwrapped }));
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
        if let Some(ref unwrapped) = self.product_code {
            r = r.push_json(json!({ "product_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.origin {
            r = r.push_json(json!({ "origin" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.start_date {
            r = r.push_json(json!({ "start_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_json(json!({ "end_date" : unwrapped }));
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
    pub fn tax_inclusive(mut self, tax_inclusive: bool) -> Self {
        self.tax_inclusive = Some(tax_inclusive);
        self
    }
    pub fn quantity(mut self, quantity: i64) -> Self {
        self.quantity = Some(quantity);
        self
    }
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());
        self
    }
    pub fn item_code(mut self, item_code: &str) -> Self {
        self.item_code = Some(item_code.to_owned());
        self
    }
    pub fn item_id(mut self, item_id: &str) -> Self {
        self.item_id = Some(item_id.to_owned());
        self
    }
    pub fn revenue_schedule_type(mut self, revenue_schedule_type: &str) -> Self {
        self.revenue_schedule_type = Some(revenue_schedule_type.to_owned());
        self
    }
    pub fn credit_reason_code(mut self, credit_reason_code: &str) -> Self {
        self.credit_reason_code = Some(credit_reason_code.to_owned());
        self
    }
    pub fn accounting_code(mut self, accounting_code: &str) -> Self {
        self.accounting_code = Some(accounting_code.to_owned());
        self
    }
    pub fn tax_exempt(mut self, tax_exempt: bool) -> Self {
        self.tax_exempt = Some(tax_exempt);
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
    pub fn product_code(mut self, product_code: &str) -> Self {
        self.product_code = Some(product_code.to_owned());
        self
    }
    pub fn origin(mut self, origin: &str) -> Self {
        self.origin = Some(origin.to_owned());
        self
    }
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.start_date = Some(start_date.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
}
pub struct CreateLineItemRequired<'a> {
    pub account_id: &'a str,
    pub currency: &'a str,
    pub unit_amount: f64,
    pub type_: &'a str,
}
impl<'a> CreateLineItemRequired<'a> {}
