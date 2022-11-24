#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        free_trial_unit: "your free trial unit",
        duration: "your duration",
        max_redemptions_per_account: 1,
        redemption_resource: "your redemption resource",
        name: "your name",
        applies_to_non_plan_charges: true,
        item_codes: &["your item codes"],
        plan_codes: &["your plan codes"],
        hosted_description: "your hosted description",
        applies_to_all_plans: true,
        temporal_unit: "your temporal unit",
        coupon_type: "your coupon type",
        unique_code_template: "your unique code template",
        discount_percent: 1,
        redeem_by_date: "your redeem by date",
        code: "your code",
        discount_type: "your discount type",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        applies_to_all_items: true,
        temporal_amount: 1,
        free_trial_amount: 1,
        max_redemptions: 1,
        invoice_description: "your invoice description",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
