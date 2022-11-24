#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        unique_code_template: "your unique code template",
        temporal_amount: 1,
        redeem_by_date: "your redeem by date",
        max_redemptions_per_account: 1,
        hosted_description: "your hosted description",
        applies_to_all_plans: true,
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        applies_to_non_plan_charges: true,
        discount_percent: 1,
        free_trial_unit: "your free trial unit",
        plan_codes: &["your plan codes"],
        temporal_unit: "your temporal unit",
        coupon_type: "your coupon type",
        applies_to_all_items: true,
        duration: "your duration",
        redemption_resource: "your redemption resource",
        item_codes: &["your item codes"],
        name: "your name",
        free_trial_amount: 1,
        code: "your code",
        max_redemptions: 1,
        discount_type: "your discount type",
        invoice_description: "your invoice description",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
