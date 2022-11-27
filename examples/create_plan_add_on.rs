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
                AddOnPricing { unit_amount : Some(1.0), currency : "your currency"
                .to_owned(), tax_inclusive : Some(true), unit_amount_decimal :
                Some("your unit amount decimal".to_owned()) }
            ],
        )
        .tier_type("your tier type")
        .usage_timeframe("your usage timeframe")
        .tiers(
            vec![
                Tier { ending_quantity : Some(1), usage_percentage :
                Some("your usage percentage".to_owned()), currencies :
                Some(vec![TierPricing { unit_amount : Some(1.0), unit_amount_decimal :
                Some("your unit amount decimal".to_owned()), currency : "your currency"
                .to_owned() }]) }
            ],
        )
        .percentage_tiers(
            vec![
                PercentageTiersByCurrency { currency : Some("your currency".to_owned()),
                tiers : Some(vec![PercentageTier { ending_amount : Some(1.0),
                usage_percentage : Some("your usage percentage".to_owned()) }]) }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
