#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        applies_to_all_items: true,
        currencies: vec![
            CouponPricing { discount : Some(1.0), currency : Some("your currency"
            .to_owned()) }
        ],
        coupon_type: "your coupon type",
        redeem_by_date: "your redeem by date",
        applies_to_all_plans: true,
        item_codes: &["your item codes"],
        duration: "your duration",
        temporal_unit: "your temporal unit",
        hosted_description: "your hosted description",
        discount_percent: 1,
        unique_code_template: "your unique code template",
        code: "your code",
        name: "your name",
        discount_type: "your discount type",
        plan_codes: &["your plan codes"],
        temporal_amount: 1,
        free_trial_amount: 1,
        redemption_resource: "your redemption resource",
        applies_to_non_plan_charges: true,
        max_redemptions: 1,
        free_trial_unit: "your free trial unit",
        invoice_description: "your invoice description",
        max_redemptions_per_account: 1,
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
