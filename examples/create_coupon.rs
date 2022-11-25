#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        name: "your name",
        unique_code_template: "your unique code template",
        max_redemptions: 1,
        duration: "your duration",
        redemption_resource: "your redemption resource",
        temporal_unit: "your temporal unit",
        free_trial_amount: 1,
        invoice_description: "your invoice description",
        applies_to_non_plan_charges: true,
        free_trial_unit: "your free trial unit",
        currencies: vec![
            CouponPricing { discount : Some(1.0), currency : Some("your currency"
            .to_owned()) }
        ],
        plan_codes: &["your plan codes"],
        discount_type: "your discount type",
        item_codes: &["your item codes"],
        max_redemptions_per_account: 1,
        redeem_by_date: "your redeem by date",
        coupon_type: "your coupon type",
        code: "your code",
        applies_to_all_items: true,
        discount_percent: 1,
        applies_to_all_plans: true,
        hosted_description: "your hosted description",
        temporal_amount: 1,
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
