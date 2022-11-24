#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        hosted_description: "your hosted description",
        redeem_by_date: "your redeem by date",
        currencies: vec![
            CouponPricing { discount : Some(1.0), currency : Some("your currency"
            .to_owned()) }
        ],
        free_trial_unit: "your free trial unit",
        name: "your name",
        applies_to_non_plan_charges: true,
        applies_to_all_plans: true,
        item_codes: &["your item codes"],
        coupon_type: "your coupon type",
        applies_to_all_items: true,
        code: "your code",
        discount_type: "your discount type",
        temporal_amount: 1,
        invoice_description: "your invoice description",
        discount_percent: 1,
        max_redemptions_per_account: 1,
        duration: "your duration",
        temporal_unit: "your temporal unit",
        unique_code_template: "your unique code template",
        redemption_resource: "your redemption resource",
        max_redemptions: 1,
        plan_codes: &["your plan codes"],
        free_trial_amount: 1,
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
