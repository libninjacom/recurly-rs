#![allow(unused_imports)]
use recurly::RecurlyClient;
use recurly::model::*;
use recurly::request::CreateCouponRequired;
#[tokio::main]
async fn main() {
    let client = RecurlyClient::from_env();
    let args = CreateCouponRequired {
        temporal_unit: "your temporal unit",
        max_redemptions: 1,
        item_codes: &["your item codes"],
        free_trial_unit: "your free trial unit",
        coupon_type: "your coupon type",
        discount_type: "your discount type",
        name: "your name",
        discount_percent: 1,
        currencies: vec![
            CouponPricing { discount : Some(1.0), currency : Some("your currency"
            .to_owned()) }
        ],
        redeem_by_date: "your redeem by date",
        temporal_amount: 1,
        invoice_description: "your invoice description",
        free_trial_amount: 1,
        applies_to_all_items: true,
        unique_code_template: "your unique code template",
        duration: "your duration",
        plan_codes: &["your plan codes"],
        hosted_description: "your hosted description",
        redemption_resource: "your redemption resource",
        code: "your code",
        max_redemptions_per_account: 1,
        applies_to_all_plans: true,
        applies_to_non_plan_charges: true,
    };
    let response = client.create_coupon(args).send().await.unwrap();
    println!("{:#?}", response);
}
