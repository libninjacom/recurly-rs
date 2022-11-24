#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        temporal_unit: "your temporal unit",
        discount_percent: 1,
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        applies_to_non_plan_charges: true,
        max_redemptions: 1,
        name: "your name",
        duration: "your duration",
        redeem_by_date: "your redeem by date",
        item_codes: &["your item codes"],
        unique_code_template: "your unique code template",
        coupon_type: "your coupon type",
        invoice_description: "your invoice description",
        temporal_amount: 1,
        applies_to_all_items: true,
        free_trial_amount: 1,
        hosted_description: "your hosted description",
        plan_codes: &["your plan codes"],
        redemption_resource: "your redemption resource",
        free_trial_unit: "your free trial unit",
        code: "your code",
        discount_type: "your discount type",
        applies_to_all_plans: true,
        max_redemptions_per_account: 1,
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
