#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        plan_codes: &["your plan codes"],
        temporal_amount: 1,
        unique_code_template: "your unique code template",
        redemption_resource: "your redemption resource",
        duration: "your duration",
        temporal_unit: "your temporal unit",
        discount_percent: 1,
        applies_to_all_plans: true,
        applies_to_all_items: true,
        applies_to_non_plan_charges: true,
        hosted_description: "your hosted description",
        code: "your code",
        discount_type: "your discount type",
        free_trial_amount: 1,
        max_redemptions: 1,
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        max_redemptions_per_account: 1,
        item_codes: &["your item codes"],
        coupon_type: "your coupon type",
        invoice_description: "your invoice description",
        free_trial_unit: "your free trial unit",
        redeem_by_date: "your redeem by date",
        name: "your name",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
