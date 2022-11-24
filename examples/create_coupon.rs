#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        free_trial_unit: "your free trial unit",
        coupon_type: "your coupon type",
        max_redemptions: 1,
        code: "your code",
        hosted_description: "your hosted description",
        redeem_by_date: "your redeem by date",
        free_trial_amount: 1,
        applies_to_all_items: true,
        temporal_unit: "your temporal unit",
        discount_type: "your discount type",
        invoice_description: "your invoice description",
        applies_to_non_plan_charges: true,
        plan_codes: &["your plan codes"],
        currencies: vec![
            CouponPricing { discount : Some(1.0), currency : Some("your currency"
            .to_owned()) }
        ],
        applies_to_all_plans: true,
        discount_percent: 1,
        item_codes: &["your item codes"],
        temporal_amount: 1,
        unique_code_template: "your unique code template",
        redemption_resource: "your redemption resource",
        name: "your name",
        duration: "your duration",
        max_redemptions_per_account: 1,
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
