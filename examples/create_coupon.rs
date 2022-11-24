#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        applies_to_non_plan_charges: true,
        applies_to_all_items: true,
        free_trial_unit: "your free trial unit",
        temporal_unit: "your temporal unit",
        redeem_by_date: "your redeem by date",
        applies_to_all_plans: true,
        coupon_type: "your coupon type",
        discount_type: "your discount type",
        free_trial_amount: 1,
        max_redemptions: 1,
        plan_codes: &["your plan codes"],
        temporal_amount: 1,
        redemption_resource: "your redemption resource",
        hosted_description: "your hosted description",
        max_redemptions_per_account: 1,
        duration: "your duration",
        code: "your code",
        unique_code_template: "your unique code template",
        discount_percent: 1,
        invoice_description: "your invoice description",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        name: "your name",
        item_codes: &["your item codes"],
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
