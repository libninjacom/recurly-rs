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
        name: "your name",
        redemption_resource: "your redemption resource",
        coupon_type: "your coupon type",
        plan_codes: &["your plan codes"],
        hosted_description: "your hosted description",
        free_trial_amount: 1,
        duration: "your duration",
        redeem_by_date: "your redeem by date",
        discount_percent: 1,
        applies_to_non_plan_charges: true,
        item_codes: &["your item codes"],
        temporal_amount: 1,
        max_redemptions: 1,
        temporal_unit: "your temporal unit",
        code: "your code",
        applies_to_all_items: true,
        unique_code_template: "your unique code template",
        currencies: vec![
            CouponPricing { discount : Some(1.0), currency : Some("your currency"
            .to_owned()) }
        ],
        applies_to_all_plans: true,
        max_redemptions_per_account: 1,
        discount_type: "your discount type",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
