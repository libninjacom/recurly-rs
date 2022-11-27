#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        discount_percent: 1,
        invoice_description: "your invoice description",
        discount_type: "your discount type",
        applies_to_all_items: true,
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        max_redemptions_per_account: 1,
        temporal_amount: 1,
        temporal_unit: "your temporal unit",
        item_codes: &["your item codes"],
        applies_to_all_plans: true,
        free_trial_amount: 1,
        max_redemptions: 1,
        redeem_by_date: "your redeem by date",
        free_trial_unit: "your free trial unit",
        duration: "your duration",
        name: "your name",
        applies_to_non_plan_charges: true,
        unique_code_template: "your unique code template",
        coupon_type: "your coupon type",
        code: "your code",
        redemption_resource: "your redemption resource",
        hosted_description: "your hosted description",
        plan_codes: &["your plan codes"],
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
