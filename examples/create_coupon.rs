#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        discount_percent: 1,
        coupon_type: "your coupon type",
        temporal_amount: 1,
        free_trial_amount: 1,
        applies_to_all_items: true,
        temporal_unit: "your temporal unit",
        discount_type: "your discount type",
        unique_code_template: "your unique code template",
        redeem_by_date: "your redeem by date",
        code: "your code",
        redemption_resource: "your redemption resource",
        duration: "your duration",
        item_codes: &["your item codes"],
        max_redemptions: 1,
        plan_codes: &["your plan codes"],
        applies_to_all_plans: true,
        name: "your name",
        applies_to_non_plan_charges: true,
        free_trial_unit: "your free trial unit",
        hosted_description: "your hosted description",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        invoice_description: "your invoice description",
        max_redemptions_per_account: 1,
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
