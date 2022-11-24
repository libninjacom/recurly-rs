#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let plan_id = "your plan id";
    let response = client
        .update_plan(plan_id)
        .id("your id")
        .code("your code")
        .name("your name")
        .description("your description")
        .accounting_code("your accounting code")
        .trial_unit("your trial unit")
        .trial_length(1)
        .trial_requires_billing_info(true)
        .total_billing_cycles(1)
        .auto_renew(true)
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
        .currencies(
            vec![
                PlanPricing { unit_amount : Some(1.0), currency : Some("your currency"
                .to_owned()), setup_fee : Some(1.0), tax_inclusive : Some(true) }
            ],
        )
        .hosted_pages(PlanHostedPages {
            success_url: Some("your success url".to_owned()),
            display_quantity: Some(true),
            bypass_confirmation: Some(true),
            cancel_url: Some("your cancel url".to_owned()),
        })
        .allow_any_item_on_subscriptions(true)
        .dunning_campaign_id("your dunning campaign id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
