#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        applies_to_all_items: true,
        item_codes: &["your item codes"],
        temporal_amount: 1,
        plan_codes: &["your plan codes"],
        temporal_unit: "your temporal unit",
        coupon_type: "your coupon type",
        hosted_description: "your hosted description",
        unique_code_template: "your unique code template",
        discount_percent: 1,
        redeem_by_date: "your redeem by date",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        duration: "your duration",
        applies_to_all_plans: true,
        invoice_description: "your invoice description",
        code: "your code",
        free_trial_unit: "your free trial unit",
        redemption_resource: "your redemption resource",
        free_trial_amount: 1,
        name: "your name",
        applies_to_non_plan_charges: true,
        max_redemptions: 1,
        max_redemptions_per_account: 1,
        discount_type: "your discount type",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
