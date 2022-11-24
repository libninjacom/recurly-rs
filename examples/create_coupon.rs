#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        coupon_type: "your coupon type",
        applies_to_non_plan_charges: true,
        item_codes: &["your item codes"],
        applies_to_all_items: true,
        redemption_resource: "your redemption resource",
        max_redemptions: 1,
        name: "your name",
        redeem_by_date: "your redeem by date",
        applies_to_all_plans: true,
        temporal_amount: 1,
        discount_percent: 1,
        free_trial_unit: "your free trial unit",
        temporal_unit: "your temporal unit",
        unique_code_template: "your unique code template",
        plan_codes: &["your plan codes"],
        duration: "your duration",
        invoice_description: "your invoice description",
        max_redemptions_per_account: 1,
        hosted_description: "your hosted description",
        code: "your code",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        discount_type: "your discount type",
        free_trial_amount: 1,
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
