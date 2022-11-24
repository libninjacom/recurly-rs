#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        plan_codes: &["your plan codes"],
        invoice_description: "your invoice description",
        hosted_description: "your hosted description",
        name: "your name",
        redeem_by_date: "your redeem by date",
        free_trial_amount: 1,
        applies_to_all_plans: true,
        coupon_type: "your coupon type",
        applies_to_non_plan_charges: true,
        free_trial_unit: "your free trial unit",
        max_redemptions: 1,
        item_codes: &["your item codes"],
        currencies: vec![
            CouponPricing { discount : Some(1.0), currency : Some("your currency"
            .to_owned()) }
        ],
        discount_type: "your discount type",
        applies_to_all_items: true,
        code: "your code",
        unique_code_template: "your unique code template",
        temporal_amount: 1,
        redemption_resource: "your redemption resource",
        max_redemptions_per_account: 1,
        discount_percent: 1,
        temporal_unit: "your temporal unit",
        duration: "your duration",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
