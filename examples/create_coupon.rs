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
        free_trial_unit: "your free trial unit",
        max_redemptions: 1,
        applies_to_non_plan_charges: true,
        invoice_description: "your invoice description",
        discount_percent: 1,
        free_trial_amount: 1,
        applies_to_all_plans: true,
        redeem_by_date: "your redeem by date",
        temporal_unit: "your temporal unit",
        code: "your code",
        applies_to_all_items: true,
        item_codes: &["your item codes"],
        duration: "your duration",
        unique_code_template: "your unique code template",
        max_redemptions_per_account: 1,
        discount_type: "your discount type",
        temporal_amount: 1,
        name: "your name",
        plan_codes: &["your plan codes"],
        hosted_description: "your hosted description",
        coupon_type: "your coupon type",
        redemption_resource: "your redemption resource",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
