#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        free_trial_unit: "your free trial unit",
        discount_percent: 1,
        code: "your code",
        unique_code_template: "your unique code template",
        invoice_description: "your invoice description",
        applies_to_all_items: true,
        redeem_by_date: "your redeem by date",
        coupon_type: "your coupon type",
        discount_type: "your discount type",
        item_codes: &["your item codes"],
        max_redemptions_per_account: 1,
        hosted_description: "your hosted description",
        temporal_unit: "your temporal unit",
        free_trial_amount: 1,
        applies_to_non_plan_charges: true,
        temporal_amount: 1,
        plan_codes: &["your plan codes"],
        name: "your name",
        applies_to_all_plans: true,
        redemption_resource: "your redemption resource",
        max_redemptions: 1,
        duration: "your duration",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
