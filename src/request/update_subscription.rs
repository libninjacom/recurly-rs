use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateSubscriptionRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub subscription_id: String,
    pub collection_method: Option<String>,
    pub custom_fields: Option<CustomFields>,
    pub remaining_billing_cycles: Option<i64>,
    pub renewal_billing_cycles: Option<i64>,
    pub auto_renew: Option<bool>,
    pub next_bill_date: Option<String>,
    pub revenue_schedule_type: Option<String>,
    pub terms_and_conditions: Option<String>,
    pub customer_notes: Option<String>,
    pub po_number: Option<String>,
    pub net_terms: Option<i64>,
    pub gateway_code: Option<String>,
    pub tax_inclusive: Option<bool>,
    pub shipping: Option<SubscriptionShippingUpdate>,
    pub billing_info_id: Option<String>,
}
impl<'a> UpdateSubscriptionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Subscription> {
        let mut r = self
            .client
            .client
            .put(
                &format!(
                    "/subscriptions/{subscription_id}", subscription_id = self
                    .subscription_id
                ),
            );
        if let Some(ref unwrapped) = self.collection_method {
            r = r.push_json(json!({ "collection_method" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.custom_fields {
            r = r.push_json(json!({ "custom_fields" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.remaining_billing_cycles {
            r = r.push_json(json!({ "remaining_billing_cycles" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.renewal_billing_cycles {
            r = r.push_json(json!({ "renewal_billing_cycles" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.auto_renew {
            r = r.push_json(json!({ "auto_renew" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.next_bill_date {
            r = r.push_json(json!({ "next_bill_date" : unwrapped }));
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
        if let Some(ref unwrapped) = self.po_number {
            r = r.push_json(json!({ "po_number" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.net_terms {
            r = r.push_json(json!({ "net_terms" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.gateway_code {
            r = r.push_json(json!({ "gateway_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tax_inclusive {
            r = r.push_json(json!({ "tax_inclusive" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.shipping {
            r = r.push_json(json!({ "shipping" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.billing_info_id {
            r = r.push_json(json!({ "billing_info_id" : unwrapped }));
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
    pub fn collection_method(mut self, collection_method: &str) -> Self {
        self.collection_method = Some(collection_method.to_owned());
        self
    }
    pub fn custom_fields(mut self, custom_fields: CustomFields) -> Self {
        self.custom_fields = Some(custom_fields);
        self
    }
    pub fn remaining_billing_cycles(mut self, remaining_billing_cycles: i64) -> Self {
        self.remaining_billing_cycles = Some(remaining_billing_cycles);
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
    pub fn next_bill_date(mut self, next_bill_date: &str) -> Self {
        self.next_bill_date = Some(next_bill_date.to_owned());
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
    pub fn po_number(mut self, po_number: &str) -> Self {
        self.po_number = Some(po_number.to_owned());
        self
    }
    pub fn net_terms(mut self, net_terms: i64) -> Self {
        self.net_terms = Some(net_terms);
        self
    }
    pub fn gateway_code(mut self, gateway_code: &str) -> Self {
        self.gateway_code = Some(gateway_code.to_owned());
        self
    }
    pub fn tax_inclusive(mut self, tax_inclusive: bool) -> Self {
        self.tax_inclusive = Some(tax_inclusive);
        self
    }
    pub fn shipping(mut self, shipping: SubscriptionShippingUpdate) -> Self {
        self.shipping = Some(shipping);
        self
    }
    pub fn billing_info_id(mut self, billing_info_id: &str) -> Self {
        self.billing_info_id = Some(billing_info_id.to_owned());
        self
    }
}
