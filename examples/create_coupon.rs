#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        max_redemptions: 1,
        duration: "your duration",
        applies_to_non_plan_charges: true,
        redemption_resource: "your redemption resource",
        invoice_description: "your invoice description",
        temporal_amount: 1,
        discount_percent: 1,
        free_trial_amount: 1,
        redeem_by_date: "your redeem by date",
        code: "your code",
        plan_codes: &["your plan codes"],
        item_codes: &["your item codes"],
        unique_code_template: "your unique code template",
        applies_to_all_plans: true,
        applies_to_all_items: true,
        temporal_unit: "your temporal unit",
        discount_type: "your discount type",
        coupon_type: "your coupon type",
        name: "your name",
        free_trial_unit: "your free trial unit",
        max_redemptions_per_account: 1,
        hosted_description: "your hosted description",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
