#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        redeem_by_date: "your redeem by date",
        free_trial_unit: "your free trial unit",
        name: "your name",
        temporal_unit: "your temporal unit",
        unique_code_template: "your unique code template",
        discount_type: "your discount type",
        duration: "your duration",
        invoice_description: "your invoice description",
        item_codes: &["your item codes"],
        applies_to_all_items: true,
        code: "your code",
        currencies: vec![
            CouponPricing { discount : Some(1.0), currency : Some("your currency"
            .to_owned()) }
        ],
        applies_to_non_plan_charges: true,
        redemption_resource: "your redemption resource",
        max_redemptions_per_account: 1,
        free_trial_amount: 1,
        max_redemptions: 1,
        discount_percent: 1,
        applies_to_all_plans: true,
        plan_codes: &["your plan codes"],
        temporal_amount: 1,
        coupon_type: "your coupon type",
        hosted_description: "your hosted description",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
