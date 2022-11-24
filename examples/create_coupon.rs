#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        temporal_unit: "your temporal unit",
        free_trial_amount: 1,
        plan_codes: &["your plan codes"],
        coupon_type: "your coupon type",
        redemption_resource: "your redemption resource",
        redeem_by_date: "your redeem by date",
        item_codes: &["your item codes"],
        max_redemptions: 1,
        applies_to_all_plans: true,
        discount_type: "your discount type",
        discount_percent: 1,
        duration: "your duration",
        name: "your name",
        applies_to_all_items: true,
        unique_code_template: "your unique code template",
        code: "your code",
        max_redemptions_per_account: 1,
        invoice_description: "your invoice description",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        applies_to_non_plan_charges: true,
        hosted_description: "your hosted description",
        temporal_amount: 1,
        free_trial_unit: "your free trial unit",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
