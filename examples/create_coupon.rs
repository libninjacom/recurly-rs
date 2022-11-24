#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        name: "your name",
        discount_type: "your discount type",
        item_codes: &["your item codes"],
        redeem_by_date: "your redeem by date",
        redemption_resource: "your redemption resource",
        applies_to_all_plans: true,
        currencies: vec![
            CouponPricing { discount : Some(1.0), currency : Some("your currency"
            .to_owned()) }
        ],
        max_redemptions: 1,
        duration: "your duration",
        applies_to_non_plan_charges: true,
        hosted_description: "your hosted description",
        discount_percent: 1,
        free_trial_amount: 1,
        invoice_description: "your invoice description",
        code: "your code",
        applies_to_all_items: true,
        plan_codes: &["your plan codes"],
        max_redemptions_per_account: 1,
        temporal_unit: "your temporal unit",
        free_trial_unit: "your free trial unit",
        coupon_type: "your coupon type",
        temporal_amount: 1,
        unique_code_template: "your unique code template",
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
