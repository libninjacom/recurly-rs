use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateShippingAddressRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub account_id: String,
    pub nickname: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub company: Option<String>,
    pub email: Option<String>,
    pub vat_number: Option<String>,
    pub phone: Option<String>,
    pub street1: String,
    pub street2: Option<String>,
    pub city: String,
    pub region: Option<String>,
    pub postal_code: String,
    pub country: String,
}
impl<'a> CreateShippingAddressRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ShippingAddress> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/accounts/{account_id}/shipping_addresses", account_id = self
                    .account_id
                ),
            );
        if let Some(ref unwrapped) = self.nickname {
            r = r.push_json(json!({ "nickname" : unwrapped }));
        }
        r = r.push_json(json!({ "first_name" : self.first_name }));
        r = r.push_json(json!({ "last_name" : self.last_name }));
        if let Some(ref unwrapped) = self.company {
            r = r.push_json(json!({ "company" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.email {
            r = r.push_json(json!({ "email" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.vat_number {
            r = r.push_json(json!({ "vat_number" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.phone {
            r = r.push_json(json!({ "phone" : unwrapped }));
        }
        r = r.push_json(json!({ "street1" : self.street1 }));
        if let Some(ref unwrapped) = self.street2 {
            r = r.push_json(json!({ "street2" : unwrapped }));
        }
        r = r.push_json(json!({ "city" : self.city }));
        if let Some(ref unwrapped) = self.region {
            r = r.push_json(json!({ "region" : unwrapped }));
        }
        r = r.push_json(json!({ "postal_code" : self.postal_code }));
        r = r.push_json(json!({ "country" : self.country }));
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
    pub fn nickname(mut self, nickname: &str) -> Self {
        self.nickname = Some(nickname.to_owned());
        self
    }
    pub fn company(mut self, company: &str) -> Self {
        self.company = Some(company.to_owned());
        self
    }
    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_owned());
        self
    }
    pub fn vat_number(mut self, vat_number: &str) -> Self {
        self.vat_number = Some(vat_number.to_owned());
        self
    }
    pub fn phone(mut self, phone: &str) -> Self {
        self.phone = Some(phone.to_owned());
        self
    }
    pub fn street2(mut self, street2: &str) -> Self {
        self.street2 = Some(street2.to_owned());
        self
    }
    pub fn region(mut self, region: &str) -> Self {
        self.region = Some(region.to_owned());
        self
    }
}
pub struct CreateShippingAddressRequired<'a> {
    pub account_id: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub street1: &'a str,
    pub city: &'a str,
    pub postal_code: &'a str,
    pub country: &'a str,
}
impl<'a> CreateShippingAddressRequired<'a> {}
