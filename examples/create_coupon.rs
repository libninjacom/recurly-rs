#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        discount_percent: 1,
        applies_to_non_plan_charges: true,
        applies_to_all_items: true,
        redeem_by_date: "your redeem by date",
        free_trial_unit: "your free trial unit",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        temporal_amount: 1,
        unique_code_template: "your unique code template",
        duration: "your duration",
        name: "your name",
        max_redemptions_per_account: 1,
        max_redemptions: 1,
        item_codes: &["your item codes"],
        coupon_type: "your coupon type",
        invoice_description: "your invoice description",
        applies_to_all_plans: true,
        hosted_description: "your hosted description",
        free_trial_amount: 1,
        plan_codes: &["your plan codes"],
        discount_type: "your discount type",
        redemption_resource: "your redemption resource",
        code: "your code",
        temporal_unit: "your temporal unit",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
