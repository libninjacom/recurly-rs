#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        name: "your name",
        duration: "your duration",
        redeem_by_date: "your redeem by date",
        unique_code_template: "your unique code template",
        item_codes: &["your item codes"],
        discount_type: "your discount type",
        temporal_unit: "your temporal unit",
        applies_to_all_plans: true,
        hosted_description: "your hosted description",
        redemption_resource: "your redemption resource",
        free_trial_amount: 1,
        plan_codes: &["your plan codes"],
        invoice_description: "your invoice description",
        coupon_type: "your coupon type",
        max_redemptions_per_account: 1,
        max_redemptions: 1,
        discount_percent: 1,
        free_trial_unit: "your free trial unit",
        temporal_amount: 1,
        applies_to_all_items: true,
        code: "your code",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        applies_to_non_plan_charges: true,
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
