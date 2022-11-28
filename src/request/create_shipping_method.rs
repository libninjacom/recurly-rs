use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateShippingMethodRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub code: String,
    pub name: String,
    pub accounting_code: Option<String>,
    pub tax_code: Option<String>,
}
impl<'a> CreateShippingMethodRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ShippingMethod> {
        let mut r = self.http_client.client.post("/shipping_methods");
        r = r.push_json(json!({ "code" : self.code }));
        r = r.push_json(json!({ "name" : self.name }));
        if let Some(ref unwrapped) = self.accounting_code {
            r = r.push_json(json!({ "accounting_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tax_code {
            r = r.push_json(json!({ "tax_code" : unwrapped }));
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
    pub fn accounting_code(mut self, accounting_code: &str) -> Self {
        self.accounting_code = Some(accounting_code.to_owned());
        self
    }
    pub fn tax_code(mut self, tax_code: &str) -> Self {
        self.tax_code = Some(tax_code.to_owned());
        self
    }
}
