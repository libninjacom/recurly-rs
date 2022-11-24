#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        max_redemptions_per_account: 1,
        hosted_description: "your hosted description",
        free_trial_unit: "your free trial unit",
        applies_to_non_plan_charges: true,
        max_redemptions: 1,
        discount_percent: 1,
        item_codes: &["your item codes"],
        temporal_amount: 1,
        duration: "your duration",
        temporal_unit: "your temporal unit",
        unique_code_template: "your unique code template",
        plan_codes: &["your plan codes"],
        applies_to_all_items: true,
        applies_to_all_plans: true,
        invoice_description: "your invoice description",
        coupon_type: "your coupon type",
        redeem_by_date: "your redeem by date",
        code: "your code",
        free_trial_amount: 1,
        redemption_resource: "your redemption resource",
        currencies: vec![
            CouponPricing { currency : Some("your currency".to_owned()), discount :
            Some(1.0) }
        ],
        discount_type: "your discount type",
        name: "your name",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
