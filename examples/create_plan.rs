#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let code = "your code";
    let name = "your name";
    let currencies = vec![
        PlanPricing { unit_amount : Some(1.0), tax_inclusive : Some(true), setup_fee :
        Some(1.0), currency : Some("your currency".to_owned()) }
    ];
    let response = client
        .create_plan(code, name, currencies)
        .description("your description")
        .accounting_code("your accounting code")
        .interval_unit("your interval unit")
        .interval_length(1)
        .trial_unit("your trial unit")
        .trial_length(1)
        .trial_requires_billing_info(true)
        .total_billing_cycles(1)
        .auto_renew(true)
        .pricing_model("your pricing model")
        .ramp_intervals(
            vec![
                PlanRampInterval { currencies : Some(vec![PlanRampPricing { unit_amount :
                1.0, currency : "your currency".to_owned() }]), starting_billing_cycle :
                Some(1) }
            ],
        )
        .custom_fields(
            CustomFields(
                vec![
                    CustomField { name : "your name".to_owned(), value : "your value"
                    .to_owned() }
                ],
            ),
        )
        .revenue_schedule_type("your revenue schedule type")
        .setup_fee_revenue_schedule_type("your setup fee revenue schedule type")
        .setup_fee_accounting_code("your setup fee accounting code")
        .avalara_transaction_type(1)
        .avalara_service_type(1)
        .tax_code("your tax code")
        .tax_exempt(true)
        .hosted_pages(PlanHostedPages {
            cancel_url: Some("your cancel url".to_owned()),
            bypass_confirmation: Some(true),
            success_url: Some("your success url".to_owned()),
            display_quantity: Some(true),
        })
        .add_ons(
            vec![
                AddOnCreate { plan_id : Some("your plan id".to_owned()), measured_unit_id
                : Some("your measured unit id".to_owned()), measured_unit_name :
                Some("your measured unit name".to_owned()), avalara_service_type :
                Some(1), currencies : Some(vec![AddOnPricing { tax_inclusive :
                Some(true), unit_amount_decimal : Some("your unit amount decimal"
                .to_owned()), unit_amount : Some(1.0), currency : "your currency"
                .to_owned() }]), optional : Some(true), display_quantity : Some(true),
                item_id : Some("your item id".to_owned()), default_quantity : Some(1),
                tier_type : Some("your tier type".to_owned()), usage_type :
                Some("your usage type".to_owned()), usage_timeframe :
                Some("your usage timeframe".to_owned()), percentage_tiers :
                Some(vec![PercentageTiersByCurrency { currency : Some("your currency"
                .to_owned()), tiers : Some(vec![PercentageTier { ending_amount :
                Some(1.0), usage_percentage : Some("your usage percentage".to_owned())
                }]) }]), tiers : Some(vec![Tier { currencies : Some(vec![TierPricing {
                unit_amount : Some(1.0), unit_amount_decimal :
                Some("your unit amount decimal".to_owned()), currency : "your currency"
                .to_owned() }]), usage_percentage : Some("your usage percentage"
                .to_owned()), ending_quantity : Some(1) }]), add_on_type :
                Some("your add on type".to_owned()), item_code : Some("your item code"
                .to_owned()), usage_calculation_type : Some("your usage calculation type"
                .to_owned()), revenue_schedule_type : Some("your revenue schedule type"
                .to_owned()), tax_code : Some("your tax code".to_owned()), code :
                "your code".to_owned(), name : "your name".to_owned(), accounting_code :
                Some("your accounting code".to_owned()), avalara_transaction_type :
                Some(1), usage_percentage : Some(1.0) }
            ],
        )
        .allow_any_item_on_subscriptions(true)
        .dunning_campaign_id("your dunning campaign id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
