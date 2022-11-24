#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateLineItemRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateLineItemRequired {
        account_id: "your account id",
        unit_amount: 1.0,
        type_: "your type",
        currency: "your currency",
    };
    let response = client
        .create_line_item(args)
        .tax_inclusive(true)
        .quantity(1)
        .description("your description")
        .item_code("your item code")
        .item_id("your item id")
        .revenue_schedule_type("your revenue schedule type")
        .credit_reason_code("your credit reason code")
        .accounting_code("your accounting code")
        .tax_exempt(true)
        .avalara_transaction_type(1)
        .avalara_service_type(1)
        .tax_code("your tax code")
        .product_code("your product code")
        .origin("your origin")
        .start_date("your start date")
        .end_date("your end date")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
