#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        hosted_description: "your hosted description",
        duration: "your duration",
        temporal_unit: "your temporal unit",
        name: "your name",
        max_redemptions: 1,
        currencies: vec![
            CouponPricing { discount : Some(1.0), currency : Some("your currency"
            .to_owned()) }
        ],
        applies_to_all_items: true,
        plan_codes: &["your plan codes"],
        max_redemptions_per_account: 1,
        item_codes: &["your item codes"],
        coupon_type: "your coupon type",
        free_trial_unit: "your free trial unit",
        discount_type: "your discount type",
        applies_to_all_plans: true,
        unique_code_template: "your unique code template",
        redeem_by_date: "your redeem by date",
        temporal_amount: 1,
        redemption_resource: "your redemption resource",
        applies_to_non_plan_charges: true,
        discount_percent: 1,
        free_trial_amount: 1,
        invoice_description: "your invoice description",
        code: "your code",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
