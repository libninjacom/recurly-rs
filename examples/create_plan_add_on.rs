#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let plan_id = "your plan id";
    let code = "your code";
    let name = "your name";
    let response = client
        .create_plan_add_on(plan_id, code, name)
        .item_code("your item code")
        .item_id("your item id")
        .add_on_type("your add on type")
        .usage_type("your usage type")
        .usage_calculation_type("your usage calculation type")
        .usage_percentage(1.0)
        .measured_unit_id("your measured unit id")
        .measured_unit_name("your measured unit name")
        .accounting_code("your accounting code")
        .revenue_schedule_type("your revenue schedule type")
        .display_quantity(true)
        .default_quantity(1)
        .optional(true)
        .avalara_transaction_type(1)
        .avalara_service_type(1)
        .tax_code("your tax code")
        .currencies(
            vec![
                AddOnPricing { currency : "your currency".to_owned(), unit_amount_decimal
                : Some("your unit amount decimal".to_owned()), unit_amount : Some(1.0),
                tax_inclusive : Some(true) }
            ],
        )
        .tier_type("your tier type")
        .usage_timeframe("your usage timeframe")
        .tiers(
            vec![
                Tier { usage_percentage : Some("your usage percentage".to_owned()),
                currencies : Some(vec![TierPricing { currency : "your currency"
                .to_owned(), unit_amount_decimal : Some("your unit amount decimal"
                .to_owned()), unit_amount : Some(1.0) }]), ending_quantity : Some(1) }
            ],
        )
        .percentage_tiers(
            vec![
                PercentageTiersByCurrency { tiers : Some(vec![PercentageTier {
                ending_amount : Some(1.0), usage_percentage :
                Some("your usage percentage".to_owned()) }]), currency :
                Some("your currency".to_owned()) }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
