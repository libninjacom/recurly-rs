#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        free_trial_amount: 1,
        max_redemptions: 1,
        coupon_type: "your coupon type",
        discount_type: "your discount type",
        invoice_description: "your invoice description",
        applies_to_all_plans: true,
        code: "your code",
        applies_to_non_plan_charges: true,
        plan_codes: &["your plan codes"],
        redemption_resource: "your redemption resource",
        applies_to_all_items: true,
        temporal_unit: "your temporal unit",
        name: "your name",
        temporal_amount: 1,
        hosted_description: "your hosted description",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        item_codes: &["your item codes"],
        free_trial_unit: "your free trial unit",
        max_redemptions_per_account: 1,
        discount_percent: 1,
        duration: "your duration",
        unique_code_template: "your unique code template",
        redeem_by_date: "your redeem by date",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
