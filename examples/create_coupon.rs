#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        applies_to_all_items: true,
        redemption_resource: "your redemption resource",
        temporal_amount: 1,
        discount_percent: 1,
        name: "your name",
        hosted_description: "your hosted description",
        max_redemptions: 1,
        coupon_type: "your coupon type",
        temporal_unit: "your temporal unit",
        plan_codes: &["your plan codes"],
        item_codes: &["your item codes"],
        max_redemptions_per_account: 1,
        free_trial_unit: "your free trial unit",
        invoice_description: "your invoice description",
        applies_to_all_plans: true,
        applies_to_non_plan_charges: true,
        duration: "your duration",
        unique_code_template: "your unique code template",
        code: "your code",
        discount_type: "your discount type",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        redeem_by_date: "your redeem by date",
        free_trial_amount: 1,
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
