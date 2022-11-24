#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        unique_code_template: "your unique code template",
        free_trial_unit: "your free trial unit",
        duration: "your duration",
        invoice_description: "your invoice description",
        redemption_resource: "your redemption resource",
        discount_type: "your discount type",
        item_codes: &["your item codes"],
        applies_to_all_plans: true,
        code: "your code",
        hosted_description: "your hosted description",
        redeem_by_date: "your redeem by date",
        applies_to_non_plan_charges: true,
        plan_codes: &["your plan codes"],
        name: "your name",
        discount_percent: 1,
        free_trial_amount: 1,
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        temporal_amount: 1,
        applies_to_all_items: true,
        temporal_unit: "your temporal unit",
        max_redemptions_per_account: 1,
        max_redemptions: 1,
        coupon_type: "your coupon type",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
