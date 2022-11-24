#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        max_redemptions: 1,
        discount_percent: 1,
        redeem_by_date: "your redeem by date",
        name: "your name",
        duration: "your duration",
        temporal_amount: 1,
        coupon_type: "your coupon type",
        applies_to_all_items: true,
        unique_code_template: "your unique code template",
        code: "your code",
        plan_codes: &["your plan codes"],
        max_redemptions_per_account: 1,
        invoice_description: "your invoice description",
        free_trial_unit: "your free trial unit",
        temporal_unit: "your temporal unit",
        redemption_resource: "your redemption resource",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        free_trial_amount: 1,
        discount_type: "your discount type",
        item_codes: &["your item codes"],
        hosted_description: "your hosted description",
        applies_to_all_plans: true,
        applies_to_non_plan_charges: true,
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
