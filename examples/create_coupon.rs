#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        hosted_description: "your hosted description",
        name: "your name",
        max_redemptions: 1,
        max_redemptions_per_account: 1,
        invoice_description: "your invoice description",
        code: "your code",
        discount_type: "your discount type",
        coupon_type: "your coupon type",
        redemption_resource: "your redemption resource",
        applies_to_all_items: true,
        applies_to_non_plan_charges: true,
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        item_codes: &["your item codes"],
        free_trial_amount: 1,
        temporal_unit: "your temporal unit",
        applies_to_all_plans: true,
        unique_code_template: "your unique code template",
        discount_percent: 1,
        duration: "your duration",
        redeem_by_date: "your redeem by date",
        temporal_amount: 1,
        free_trial_unit: "your free trial unit",
        plan_codes: &["your plan codes"],
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
