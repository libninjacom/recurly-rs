#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        invoice_description: "your invoice description",
        free_trial_unit: "your free trial unit",
        discount_type: "your discount type",
        applies_to_non_plan_charges: true,
        coupon_type: "your coupon type",
        applies_to_all_items: true,
        hosted_description: "your hosted description",
        duration: "your duration",
        applies_to_all_plans: true,
        redeem_by_date: "your redeem by date",
        free_trial_amount: 1,
        temporal_unit: "your temporal unit",
        unique_code_template: "your unique code template",
        redemption_resource: "your redemption resource",
        currencies: vec![
            CouponPricing { discount : Some(1.0), currency : Some("your currency"
            .to_owned()) }
        ],
        plan_codes: &["your plan codes"],
        max_redemptions: 1,
        max_redemptions_per_account: 1,
        discount_percent: 1,
        name: "your name",
        item_codes: &["your item codes"],
        temporal_amount: 1,
        code: "your code",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
