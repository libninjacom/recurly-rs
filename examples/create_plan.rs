#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let code = "your code";
    let name = "your name";
    let currencies = vec![
        PlanPricing { currency : Some("your currency".to_owned()), tax_inclusive :
        Some(true), unit_amount : Some(1.0), setup_fee : Some(1.0) }
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
                PlanRampInterval { starting_billing_cycle : Some(1), currencies :
                Some(vec![PlanRampPricing { unit_amount : 1.0, currency : "your currency"
                .to_owned() }]) }
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
            display_quantity: Some(true),
            bypass_confirmation: Some(true),
            success_url: Some("your success url".to_owned()),
            cancel_url: Some("your cancel url".to_owned()),
        })
        .add_ons(
            vec![
                AddOnCreate { currencies : Some(vec![AddOnPricing { unit_amount :
                Some(1.0), unit_amount_decimal : Some("your unit amount decimal"
                .to_owned()), tax_inclusive : Some(true), currency : "your currency"
                .to_owned() }]), accounting_code : Some("your accounting code"
                .to_owned()), measured_unit_id : Some("your measured unit id"
                .to_owned()), item_code : Some("your item code".to_owned()), plan_id :
                Some("your plan id".to_owned()), default_quantity : Some(1), optional :
                Some(true), usage_percentage : Some(1.0), tiers : Some(vec![Tier {
                ending_quantity : Some(1), usage_percentage :
                Some("your usage percentage".to_owned()), currencies :
                Some(vec![TierPricing { currency : "your currency".to_owned(),
                unit_amount : Some(1.0), unit_amount_decimal :
                Some("your unit amount decimal".to_owned()) }]) }]), code : "your code"
                .to_owned(), usage_calculation_type : Some("your usage calculation type"
                .to_owned()), avalara_service_type : Some(1), item_id :
                Some("your item id".to_owned()), avalara_transaction_type : Some(1),
                percentage_tiers : Some(vec![PercentageTiersByCurrency { currency :
                Some("your currency".to_owned()), tiers : Some(vec![PercentageTier {
                ending_amount : Some(1.0), usage_percentage :
                Some("your usage percentage".to_owned()) }]) }]), name : "your name"
                .to_owned(), add_on_type : Some("your add on type".to_owned()),
                usage_type : Some("your usage type".to_owned()), measured_unit_name :
                Some("your measured unit name".to_owned()), revenue_schedule_type :
                Some("your revenue schedule type".to_owned()), display_quantity :
                Some(true), tier_type : Some("your tier type".to_owned()),
                usage_timeframe : Some("your usage timeframe".to_owned()), tax_code :
                Some("your tax code".to_owned()) }
            ],
        )
        .allow_any_item_on_subscriptions(true)
        .dunning_campaign_id("your dunning campaign id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
