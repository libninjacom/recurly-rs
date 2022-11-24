#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let plan_id = "your plan id";
    let add_on_id = "your add on id";
    let response = client
        .update_plan_add_on(plan_id, add_on_id)
        .id("your id")
        .code("your code")
        .name("your name")
        .usage_percentage(1.0)
        .usage_calculation_type("your usage calculation type")
        .measured_unit_id("your measured unit id")
        .measured_unit_name("your measured unit name")
        .accounting_code("your accounting code")
        .revenue_schedule_type("your revenue schedule type")
        .avalara_transaction_type(1)
        .avalara_service_type(1)
        .tax_code("your tax code")
        .display_quantity(true)
        .default_quantity(1)
        .optional(true)
        .currencies(
            vec![
                AddOnPricing { tax_inclusive : Some(true), unit_amount : Some(1.0),
                currency : "your currency".to_owned(), unit_amount_decimal :
                Some("your unit amount decimal".to_owned()) }
            ],
        )
        .tiers(
            vec![
                Tier { currencies : Some(vec![TierPricing { unit_amount : Some(1.0),
                unit_amount_decimal : Some("your unit amount decimal".to_owned()),
                currency : "your currency".to_owned() }]), usage_percentage :
                Some("your usage percentage".to_owned()), ending_quantity : Some(1) }
            ],
        )
        .percentage_tiers(
            vec![
                PercentageTiersByCurrency { currency : Some("your currency".to_owned()),
                tiers : Some(vec![PercentageTier { usage_percentage :
                Some("your usage percentage".to_owned()), ending_amount : Some(1.0) }]) }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
