#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        max_redemptions_per_account: 1,
        max_redemptions: 1,
        discount_type: "your discount type",
        hosted_description: "your hosted description",
        code: "your code",
        temporal_unit: "your temporal unit",
        redemption_resource: "your redemption resource",
        discount_percent: 1,
        applies_to_non_plan_charges: true,
        name: "your name",
        applies_to_all_items: true,
        item_codes: &["your item codes"],
        free_trial_amount: 1,
        free_trial_unit: "your free trial unit",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        temporal_amount: 1,
        invoice_description: "your invoice description",
        coupon_type: "your coupon type",
        unique_code_template: "your unique code template",
        redeem_by_date: "your redeem by date",
        applies_to_all_plans: true,
        plan_codes: &["your plan codes"],
        duration: "your duration",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
