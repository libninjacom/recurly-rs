#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        temporal_unit: "your temporal unit",
        currencies: vec![
            CouponPricing { discount : Some(1.0), currency : Some("your currency"
            .to_owned()) }
        ],
        redemption_resource: "your redemption resource",
        free_trial_unit: "your free trial unit",
        temporal_amount: 1,
        free_trial_amount: 1,
        max_redemptions: 1,
        hosted_description: "your hosted description",
        item_codes: &["your item codes"],
        duration: "your duration",
        name: "your name",
        code: "your code",
        coupon_type: "your coupon type",
        invoice_description: "your invoice description",
        max_redemptions_per_account: 1,
        redeem_by_date: "your redeem by date",
        applies_to_non_plan_charges: true,
        discount_percent: 1,
        applies_to_all_items: true,
        plan_codes: &["your plan codes"],
        applies_to_all_plans: true,
        discount_type: "your discount type",
        unique_code_template: "your unique code template",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
