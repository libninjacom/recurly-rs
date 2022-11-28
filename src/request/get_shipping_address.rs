use serde_json::json;
use crate::model::*;
use crate::RecurlyClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetShippingAddressRequest<'a> {
    pub(crate) http_client: &'a RecurlyClient,
    pub account_id: String,
    pub shipping_address_id: String,
}
impl<'a> GetShippingAddressRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ShippingAddress> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/accounts/{account_id}/shipping_addresses/{shipping_address_id}",
                    account_id = self.account_id, shipping_address_id = self
                    .shipping_address_id
                ),
            );
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
}
