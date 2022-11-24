#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        applies_to_non_plan_charges: true,
        item_codes: &["your item codes"],
        coupon_type: "your coupon type",
        hosted_description: "your hosted description",
        free_trial_amount: 1,
        applies_to_all_items: true,
        code: "your code",
        discount_type: "your discount type",
        currencies: vec![
            CouponPricing { discount : Some(1.0), currency : Some("your currency"
            .to_owned()) }
        ],
        plan_codes: &["your plan codes"],
        max_redemptions: 1,
        duration: "your duration",
        redeem_by_date: "your redeem by date",
        temporal_amount: 1,
        unique_code_template: "your unique code template",
        applies_to_all_plans: true,
        redemption_resource: "your redemption resource",
        temporal_unit: "your temporal unit",
        discount_percent: 1,
        max_redemptions_per_account: 1,
        name: "your name",
        invoice_description: "your invoice description",
        free_trial_unit: "your free trial unit",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
