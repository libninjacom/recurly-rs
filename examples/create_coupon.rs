#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        plan_codes: &["your plan codes"],
        free_trial_amount: 1,
        unique_code_template: "your unique code template",
        item_codes: &["your item codes"],
        redemption_resource: "your redemption resource",
        applies_to_all_items: true,
        max_redemptions: 1,
        temporal_unit: "your temporal unit",
        hosted_description: "your hosted description",
        applies_to_all_plans: true,
        redeem_by_date: "your redeem by date",
        free_trial_unit: "your free trial unit",
        code: "your code",
        invoice_description: "your invoice description",
        max_redemptions_per_account: 1,
        duration: "your duration",
        temporal_amount: 1,
        name: "your name",
        discount_percent: 1,
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        coupon_type: "your coupon type",
        applies_to_non_plan_charges: true,
        discount_type: "your discount type",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
