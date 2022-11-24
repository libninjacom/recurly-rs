use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateShippingAddressRequest<'a> {
    pub(crate) client: &'a RecurlyClient,
    pub account_id: String,
    pub shipping_address_id: String,
    pub id: Option<String>,
    pub nickname: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company: Option<String>,
    pub email: Option<String>,
    pub vat_number: Option<String>,
    pub phone: Option<String>,
    pub street1: Option<String>,
    pub street2: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}
impl<'a> UpdateShippingAddressRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ShippingAddress> {
        let mut r = self
            .client
            .client
            .put(
                &format!(
                    "/accounts/{account_id}/shipping_addresses/{shipping_address_id}",
                    account_id = self.account_id, shipping_address_id = self
                    .shipping_address_id
                ),
            );
        if let Some(ref unwrapped) = self.id {
            r = r.push_json(json!({ "id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.nickname {
            r = r.push_json(json!({ "nickname" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.first_name {
            r = r.push_json(json!({ "first_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.last_name {
            r = r.push_json(json!({ "last_name" : unwrapped }));
        }
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
        if let Some(ref unwrapped) = self.street1 {
            r = r.push_json(json!({ "street1" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.street2 {
            r = r.push_json(json!({ "street2" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.city {
            r = r.push_json(json!({ "city" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.region {
            r = r.push_json(json!({ "region" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.postal_code {
            r = r.push_json(json!({ "postal_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.country {
            r = r.push_json(json!({ "country" : unwrapped }));
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
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_owned());
        self
    }
    pub fn nickname(mut self, nickname: &str) -> Self {
        self.nickname = Some(nickname.to_owned());
        self
    }
    pub fn first_name(mut self, first_name: &str) -> Self {
        self.first_name = Some(first_name.to_owned());
        self
    }
    pub fn last_name(mut self, last_name: &str) -> Self {
        self.last_name = Some(last_name.to_owned());
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
    pub fn street1(mut self, street1: &str) -> Self {
        self.street1 = Some(street1.to_owned());
        self
    }
    pub fn street2(mut self, street2: &str) -> Self {
        self.street2 = Some(street2.to_owned());
        self
    }
    pub fn city(mut self, city: &str) -> Self {
        self.city = Some(city.to_owned());
        self
    }
    pub fn region(mut self, region: &str) -> Self {
        self.region = Some(region.to_owned());
        self
    }
    pub fn postal_code(mut self, postal_code: &str) -> Self {
        self.postal_code = Some(postal_code.to_owned());
        self
    }
    pub fn country(mut self, country: &str) -> Self {
        self.country = Some(country.to_owned());
        self
    }
}
