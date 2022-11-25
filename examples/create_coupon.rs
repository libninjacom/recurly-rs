#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        plan_codes: &["your plan codes"],
        max_redemptions: 1,
        discount_type: "your discount type",
        applies_to_all_items: true,
        discount_percent: 1,
        name: "your name",
        free_trial_unit: "your free trial unit",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        item_codes: &["your item codes"],
        applies_to_all_plans: true,
        max_redemptions_per_account: 1,
        hosted_description: "your hosted description",
        temporal_amount: 1,
        temporal_unit: "your temporal unit",
        invoice_description: "your invoice description",
        coupon_type: "your coupon type",
        duration: "your duration",
        unique_code_template: "your unique code template",
        code: "your code",
        redeem_by_date: "your redeem by date",
        free_trial_amount: 1,
        applies_to_non_plan_charges: true,
        redemption_resource: "your redemption resource",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
