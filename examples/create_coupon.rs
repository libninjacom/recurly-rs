#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        applies_to_non_plan_charges: true,
        max_redemptions: 1,
        redeem_by_date: "your redeem by date",
        item_codes: &["your item codes"],
        unique_code_template: "your unique code template",
        applies_to_all_plans: true,
        plan_codes: &["your plan codes"],
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        duration: "your duration",
        hosted_description: "your hosted description",
        discount_type: "your discount type",
        temporal_amount: 1,
        discount_percent: 1,
        max_redemptions_per_account: 1,
        name: "your name",
        free_trial_unit: "your free trial unit",
        free_trial_amount: 1,
        coupon_type: "your coupon type",
        code: "your code",
        redemption_resource: "your redemption resource",
        temporal_unit: "your temporal unit",
        applies_to_all_items: true,
        invoice_description: "your invoice description",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
