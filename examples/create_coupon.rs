#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        redeem_by_date: "your redeem by date",
        free_trial_amount: 1,
        discount_percent: 1,
        max_redemptions: 1,
        free_trial_unit: "your free trial unit",
        max_redemptions_per_account: 1,
        applies_to_all_items: true,
        temporal_unit: "your temporal unit",
        redemption_resource: "your redemption resource",
        item_codes: &["your item codes"],
        discount_type: "your discount type",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        plan_codes: &["your plan codes"],
        temporal_amount: 1,
        unique_code_template: "your unique code template",
        name: "your name",
        invoice_description: "your invoice description",
        duration: "your duration",
        applies_to_non_plan_charges: true,
        code: "your code",
        coupon_type: "your coupon type",
        applies_to_all_plans: true,
        hosted_description: "your hosted description",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
