#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        code: "your code",
        name: "your name",
        duration: "your duration",
        temporal_unit: "your temporal unit",
        unique_code_template: "your unique code template",
        temporal_amount: 1,
        plan_codes: &["your plan codes"],
        applies_to_all_plans: true,
        discount_type: "your discount type",
        applies_to_all_items: true,
        max_redemptions: 1,
        hosted_description: "your hosted description",
        invoice_description: "your invoice description",
        discount_percent: 1,
        max_redemptions_per_account: 1,
        free_trial_amount: 1,
        redemption_resource: "your redemption resource",
        redeem_by_date: "your redeem by date",
        coupon_type: "your coupon type",
        free_trial_unit: "your free trial unit",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        item_codes: &["your item codes"],
        applies_to_non_plan_charges: true,
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
