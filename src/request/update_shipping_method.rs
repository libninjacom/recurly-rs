use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateShippingMethodRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub shipping_method_id: String,
    pub code: Option<String>,
    pub name: Option<String>,
    pub accounting_code: Option<String>,
    pub tax_code: Option<String>,
}
impl<'a> UpdateShippingMethodRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ShippingMethod> {
        let mut r = self
            .client
            .client
            .put(
                &format!(
                    "/shipping_methods/{shipping_method_id}", shipping_method_id = self
                    .shipping_method_id
                ),
            );
        if let Some(ref unwrapped) = self.code {
            r = r.push_json(json!({ "code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.accounting_code {
            r = r.push_json(json!({ "accounting_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tax_code {
            r = r.push_json(json!({ "tax_code" : unwrapped }));
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
    pub fn accounting_code(mut self, accounting_code: &str) -> Self {
        self.accounting_code = Some(accounting_code.to_owned());
        self
    }
    pub fn tax_code(mut self, tax_code: &str) -> Self {
        self.tax_code = Some(tax_code.to_owned());
        self
    }
}
