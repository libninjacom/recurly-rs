#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        plan_codes: &["your plan codes"],
        unique_code_template: "your unique code template",
        free_trial_unit: "your free trial unit",
        currencies: vec![
            CouponPricing { discount : Some(1.0), currency : Some("your currency"
            .to_owned()) }
        ],
        redeem_by_date: "your redeem by date",
        applies_to_all_plans: true,
        applies_to_all_items: true,
        applies_to_non_plan_charges: true,
        name: "your name",
        discount_type: "your discount type",
        discount_percent: 1,
        max_redemptions_per_account: 1,
        free_trial_amount: 1,
        code: "your code",
        coupon_type: "your coupon type",
        max_redemptions: 1,
        item_codes: &["your item codes"],
        temporal_unit: "your temporal unit",
        hosted_description: "your hosted description",
        invoice_description: "your invoice description",
        duration: "your duration",
        temporal_amount: 1,
        redemption_resource: "your redemption resource",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
