#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        unique_code_template: "your unique code template",
        code: "your code",
        free_trial_amount: 1,
        name: "your name",
        redemption_resource: "your redemption resource",
        temporal_amount: 1,
        invoice_description: "your invoice description",
        hosted_description: "your hosted description",
        redeem_by_date: "your redeem by date",
        free_trial_unit: "your free trial unit",
        max_redemptions_per_account: 1,
        discount_percent: 1,
        currencies: vec![
            CouponPricing { discount : Some(1.0), currency : Some("your currency"
            .to_owned()) }
        ],
        discount_type: "your discount type",
        applies_to_non_plan_charges: true,
        applies_to_all_plans: true,
        plan_codes: &["your plan codes"],
        max_redemptions: 1,
        item_codes: &["your item codes"],
        duration: "your duration",
        applies_to_all_items: true,
        temporal_unit: "your temporal unit",
        coupon_type: "your coupon type",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
