#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        plan_codes: &["your plan codes"],
        temporal_amount: 1,
        item_codes: &["your item codes"],
        free_trial_unit: "your free trial unit",
        discount_percent: 1,
        free_trial_amount: 1,
        code: "your code",
        applies_to_all_items: true,
        name: "your name",
        hosted_description: "your hosted description",
        unique_code_template: "your unique code template",
        temporal_unit: "your temporal unit",
        max_redemptions: 1,
        redemption_resource: "your redemption resource",
        max_redemptions_per_account: 1,
        applies_to_all_plans: true,
        duration: "your duration",
        coupon_type: "your coupon type",
        invoice_description: "your invoice description",
        applies_to_non_plan_charges: true,
        redeem_by_date: "your redeem by date",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        discount_type: "your discount type",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
