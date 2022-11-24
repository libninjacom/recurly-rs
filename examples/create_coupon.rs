#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        unique_code_template: "your unique code template",
        applies_to_non_plan_charges: true,
        code: "your code",
        free_trial_unit: "your free trial unit",
        plan_codes: &["your plan codes"],
        invoice_description: "your invoice description",
        duration: "your duration",
        discount_type: "your discount type",
        name: "your name",
        redeem_by_date: "your redeem by date",
        applies_to_all_plans: true,
        temporal_unit: "your temporal unit",
        max_redemptions_per_account: 1,
        discount_percent: 1,
        max_redemptions: 1,
        item_codes: &["your item codes"],
        free_trial_amount: 1,
        temporal_amount: 1,
        coupon_type: "your coupon type",
        redemption_resource: "your redemption resource",
        applies_to_all_items: true,
        hosted_description: "your hosted description",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
