#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        duration: "your duration",
        hosted_description: "your hosted description",
        unique_code_template: "your unique code template",
        max_redemptions: 1,
        discount_type: "your discount type",
        applies_to_non_plan_charges: true,
        redeem_by_date: "your redeem by date",
        max_redemptions_per_account: 1,
        currencies: vec![
            CouponPricing { discount : Some(1.0), currency : Some("your currency"
            .to_owned()) }
        ],
        name: "your name",
        free_trial_unit: "your free trial unit",
        plan_codes: &["your plan codes"],
        temporal_unit: "your temporal unit",
        applies_to_all_items: true,
        discount_percent: 1,
        invoice_description: "your invoice description",
        free_trial_amount: 1,
        temporal_amount: 1,
        code: "your code",
        item_codes: &["your item codes"],
        applies_to_all_plans: true,
        coupon_type: "your coupon type",
        redemption_resource: "your redemption resource",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
