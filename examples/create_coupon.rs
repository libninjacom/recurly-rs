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
        max_redemptions: 1,
        temporal_amount: 1,
        unique_code_template: "your unique code template",
        coupon_type: "your coupon type",
        invoice_description: "your invoice description",
        redemption_resource: "your redemption resource",
        max_redemptions_per_account: 1,
        name: "your name",
        applies_to_all_plans: true,
        hosted_description: "your hosted description",
        free_trial_unit: "your free trial unit",
        duration: "your duration",
        code: "your code",
        redeem_by_date: "your redeem by date",
        applies_to_all_items: true,
        discount_percent: 1,
        free_trial_amount: 1,
        discount_type: "your discount type",
        temporal_unit: "your temporal unit",
        applies_to_non_plan_charges: true,
        item_codes: &["your item codes"],
        plan_codes: &["your plan codes"],
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
