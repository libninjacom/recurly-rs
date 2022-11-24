#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        max_redemptions_per_account: 1,
        free_trial_amount: 1,
        duration: "your duration",
        max_redemptions: 1,
        discount_percent: 1,
        plan_codes: &["your plan codes"],
        redemption_resource: "your redemption resource",
        applies_to_non_plan_charges: true,
        item_codes: &["your item codes"],
        discount_type: "your discount type",
        redeem_by_date: "your redeem by date",
        free_trial_unit: "your free trial unit",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        applies_to_all_items: true,
        applies_to_all_plans: true,
        temporal_unit: "your temporal unit",
        temporal_amount: 1,
        hosted_description: "your hosted description",
        invoice_description: "your invoice description",
        unique_code_template: "your unique code template",
        name: "your name",
        code: "your code",
        coupon_type: "your coupon type",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
