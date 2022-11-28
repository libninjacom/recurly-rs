use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreatePurchaseRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub currency: String,
    pub account: AccountPurchase,
    pub billing_info_id: Option<String>,
    pub collection_method: Option<String>,
    pub po_number: Option<String>,
    pub net_terms: Option<i64>,
    pub terms_and_conditions: Option<String>,
    pub customer_notes: Option<String>,
    pub vat_reverse_charge_notes: Option<String>,
    pub credit_customer_notes: Option<String>,
    pub gateway_code: Option<String>,
    pub shipping: Option<serde_json::Value>,
    pub line_items: Option<Vec<LineItemCreate>>,
    pub subscriptions: Option<Vec<SubscriptionPurchase>>,
    pub coupon_codes: Option<Vec<String>>,
    pub gift_card_redemption_code: Option<String>,
    pub transaction_type: Option<String>,
}
impl<'a> CreatePurchaseRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InvoiceCollection> {
        let mut r = self.http_client.client.post("/purchases");
        r = r.push_json(json!({ "currency" : self.currency }));
        r = r.push_json(json!({ "account" : self.account }));
        if let Some(ref unwrapped) = self.billing_info_id {
            r = r.push_json(json!({ "billing_info_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.collection_method {
            r = r.push_json(json!({ "collection_method" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.po_number {
            r = r.push_json(json!({ "po_number" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.net_terms {
            r = r.push_json(json!({ "net_terms" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.terms_and_conditions {
            r = r.push_json(json!({ "terms_and_conditions" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.customer_notes {
            r = r.push_json(json!({ "customer_notes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.vat_reverse_charge_notes {
            r = r.push_json(json!({ "vat_reverse_charge_notes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.credit_customer_notes {
            r = r.push_json(json!({ "credit_customer_notes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.gateway_code {
            r = r.push_json(json!({ "gateway_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.shipping {
            r = r.push_json(json!({ "shipping" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.line_items {
            r = r.push_json(json!({ "line_items" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.subscriptions {
            r = r.push_json(json!({ "subscriptions" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.coupon_codes {
            r = r.push_json(json!({ "coupon_codes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.gift_card_redemption_code {
            r = r.push_json(json!({ "gift_card_redemption_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.transaction_type {
            r = r.push_json(json!({ "transaction_type" : unwrapped }));
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
    pub fn billing_info_id(mut self, billing_info_id: &str) -> Self {
        self.billing_info_id = Some(billing_info_id.to_owned());
        self
    }
    pub fn collection_method(mut self, collection_method: &str) -> Self {
        self.collection_method = Some(collection_method.to_owned());
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
    pub fn terms_and_conditions(mut self, terms_and_conditions: &str) -> Self {
        self.terms_and_conditions = Some(terms_and_conditions.to_owned());
        self
    }
    pub fn customer_notes(mut self, customer_notes: &str) -> Self {
        self.customer_notes = Some(customer_notes.to_owned());
        self
    }
    pub fn vat_reverse_charge_notes(mut self, vat_reverse_charge_notes: &str) -> Self {
        self.vat_reverse_charge_notes = Some(vat_reverse_charge_notes.to_owned());
        self
    }
    pub fn credit_customer_notes(mut self, credit_customer_notes: &str) -> Self {
        self.credit_customer_notes = Some(credit_customer_notes.to_owned());
        self
    }
    pub fn gateway_code(mut self, gateway_code: &str) -> Self {
        self.gateway_code = Some(gateway_code.to_owned());
        self
    }
    pub fn shipping(mut self, shipping: serde_json::Value) -> Self {
        self.shipping = Some(shipping);
        self
    }
    pub fn line_items(mut self, line_items: Vec<LineItemCreate>) -> Self {
        self.line_items = Some(line_items);
        self
    }
    pub fn subscriptions(mut self, subscriptions: Vec<SubscriptionPurchase>) -> Self {
        self.subscriptions = Some(subscriptions);
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
    pub fn gift_card_redemption_code(mut self, gift_card_redemption_code: &str) -> Self {
        self.gift_card_redemption_code = Some(gift_card_redemption_code.to_owned());
        self
    }
    pub fn transaction_type(mut self, transaction_type: &str) -> Self {
        self.transaction_type = Some(transaction_type.to_owned());
        self
    }
}
