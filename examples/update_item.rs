#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let item_id = "your item id";
    let response = client
        .update_item(item_id)
        .code("your code")
        .name("your name")
        .description("your description")
        .external_sku("your external sku")
        .accounting_code("your accounting code")
        .revenue_schedule_type("your revenue schedule type")
        .avalara_transaction_type(1)
        .avalara_service_type(1)
        .tax_code("your tax code")
        .tax_exempt(true)
        .custom_fields(
            CustomFields(
                vec![
                    CustomField { name : "your name".to_owned(), value : "your value"
                    .to_owned() }
                ],
            ),
        )
        .currencies(
            vec![
                Pricing { currency : "your currency".to_owned(), tax_inclusive :
                Some(true), unit_amount : 1.0 }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
