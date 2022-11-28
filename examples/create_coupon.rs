#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        hosted_description: "your hosted description",
        item_codes: &["your item codes"],
        duration: "your duration",
        name: "your name",
        redeem_by_date: "your redeem by date",
        applies_to_non_plan_charges: true,
        temporal_amount: 1,
        temporal_unit: "your temporal unit",
        max_redemptions_per_account: 1,
        discount_percent: 1,
        free_trial_amount: 1,
        applies_to_all_plans: true,
        plan_codes: &["your plan codes"],
        redemption_resource: "your redemption resource",
        invoice_description: "your invoice description",
        applies_to_all_items: true,
        free_trial_unit: "your free trial unit",
        discount_type: "your discount type",
        max_redemptions: 1,
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        coupon_type: "your coupon type",
        code: "your code",
        unique_code_template: "your unique code template",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
