use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UniqueCouponCodeParams {
    pub limit: Option<i64>,
    pub order: Option<String>,
    pub sort: Option<String>,
    ///The date-time to be included when listing UniqueCouponCodes
    pub begin_time: Option<String>,
}
impl std::fmt::Display for UniqueCouponCodeParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CouponDiscountPricing {
    ///3-letter ISO 4217 currency code.
    pub currency: Option<String>,
    ///Value of the fixed discount that this coupon applies.
    pub amount: Option<f64>,
}
impl std::fmt::Display for CouponDiscountPricing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionChangeBillingInfoCreate {
    ///Accept nested attributes for three_d_secure_action_result_token_id
    pub subscription_change_billing_info: SubscriptionChangeBillingInfo,
}
impl std::fmt::Display for SubscriptionChangeBillingInfoCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DunningInterval(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionChangeCreate {
    ///A list of coupon_codes to be redeemed on the subscription during the change. Only allowed if timeframe is now and you change something about the subscription that creates an invoice.
    pub coupon_codes: Option<Vec<String>>,
    pub revenue_schedule_type: Option<String>,
    ///An optional type designation for the payment gateway transaction created by this request. Supports 'moto' value, which is the acronym for mail order and telephone transactions.
    pub transaction_type: Option<String>,
    ///If you want to change to a new plan, you can provide the plan's code or id. If both are provided the `plan_id` will be used.
    pub plan_code: Option<String>,
    ///Shipping addresses are tied to a customer's account. Each account can have up to 20 different shipping addresses, and if you have enabled multiple subscriptions per account, you can associate different shipping addresses to each subscription.
    pub shipping: Option<SubscriptionChangeShippingCreate>,
    ///For manual invoicing, this identifies the PO number associated with the subscription.
    pub po_number: Option<String>,
    ///The timeframe parameter controls when the upgrade or downgrade takes place. The subscription change can occur now, when the subscription is next billed, or when the subscription term ends. Generally, if you're performing an upgrade, you will want the change to occur immediately (now). If you're performing a downgrade, you should set the timeframe to `term_end` or `bill_date` so the change takes effect at a scheduled billing date. The `renewal` timeframe option is accepted as an alias for `term_end`.
    pub timeframe: Option<String>,
    ///If you want to change to a new plan, you can provide the plan's code or id. If both are provided the `plan_id` will be used.
    pub plan_id: Option<String>,
    pub billing_info: Option<SubscriptionChangeBillingInfoCreate>,
    ///This field is deprecated. Please do not use it.
    pub tax_inclusive: Option<bool>,
    /**If you provide a value for this field it will replace any
existing add-ons. So, when adding or modifying an add-on, you need to
include the existing subscription add-ons. Unchanged add-ons can be included
just using the subscription add-on''s ID: `{"id": "abc123"}`. If this
value is omitted your existing add-ons will be unaffected. To remove all
existing add-ons, this value should be an empty array.'

If a subscription add-on's `code` is supplied without the `id`,
`{"code": "def456"}`, the subscription add-on attributes will be set to the
current values of the plan add-on unless provided in the request.

- If an `id` is passed, any attributes not passed in will pull from the
  existing subscription add-on
- If a `code` is passed, any attributes not passed in will pull from the
  current values of the plan add-on
- Attributes passed in as part of the request will override either of the
  above scenarios
*/
    pub add_ons: Option<Vec<SubscriptionAddOnUpdate>>,
    ///Integer representing the number of days after an invoice's creation that the invoice will become past due. If an invoice's net terms are set to '0', it is due 'On Receipt' and will become past due 24 hours after it’s created. If an invoice is due net 30, it will become past due at 31 days exactly.
    pub net_terms: Option<i64>,
    pub collection_method: Option<String>,
    ///The custom fields will only be altered when they are included in a request. Sending an empty array will not remove any existing values. To remove a field send the name with a null or empty value.
    pub custom_fields: Option<CustomFields>,
    ///The new set of ramp intervals for the subscription.
    pub ramp_intervals: Option<Vec<SubscriptionRampInterval>>,
    ///Optionally, sets custom pricing for the subscription, overriding the plan's default unit amount. The subscription's current currency will be used.
    pub unit_amount: Option<f64>,
    ///Optionally override the default quantity of 1.
    pub quantity: Option<i64>,
}
impl std::fmt::Display for SubscriptionChangeCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExportFile {
    ///Name of the export file.
    pub name: Option<String>,
    ///A presigned link to download the export file.
    pub href: Option<String>,
    ///MD5 hash of the export file.
    pub md5_sum: Option<String>,
}
impl std::fmt::Display for ExportFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExternalSubscriptionList {
    ///Will always be List.
    pub object: Option<String>,
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
    pub data: Option<Vec<ExternalSubscription>>,
}
impl std::fmt::Display for ExternalSubscriptionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Error {
    pub params: Option<Vec<serde_json::Value>>,
    pub type_: Option<String>,
    pub message: Option<String>,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Entitlement {
    pub customer_permission: Option<CustomerPermission>,
    ///Subscription or item that granted the customer permission.
    pub granted_by: Option<Vec<GrantedBy>>,
    ///Time object was created.
    pub created_at: Option<String>,
    ///Time the object was last updated
    pub updated_at: Option<String>,
    ///Entitlement
    pub object: Option<String>,
}
impl std::fmt::Display for Entitlement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Empty {}
impl std::fmt::Display for Empty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UniqueCouponCodeList {
    pub data: Option<Vec<UniqueCouponCode>>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
    ///Will always be List.
    pub object: Option<String>,
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
}
impl std::fmt::Display for UniqueCouponCodeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountNote {
    pub object: Option<String>,
    pub id: Option<String>,
    pub created_at: Option<String>,
    pub account_id: Option<String>,
    pub message: String,
    pub user: Option<User>,
}
impl std::fmt::Display for AccountNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AddOnList {
    ///Will always be List.
    pub object: Option<String>,
    pub data: Option<Vec<AddOn>>,
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
}
impl std::fmt::Display for AddOnList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemCreate {
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the item is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types.
    pub avalara_service_type: Option<i64>,
    ///Unique code to identify the item.
    pub code: String,
    ///The custom fields will only be altered when they are included in a request. Sending an empty array will not remove any existing values. To remove a field send the name with a null or empty value.
    pub custom_fields: Option<CustomFields>,
    pub currencies: Option<Vec<Pricing>>,
    ///Optional, description.
    pub description: Option<String>,
    ///Used by Avalara, Vertex, and Recurly’s EU VAT tax feature. The tax code values are specific to each tax system. If you are using Recurly’s EU VAT feature you can use `unknown`, `physical`, or `digital`.
    pub tax_code: Option<String>,
    ///Accounting code for invoice line items.
    pub accounting_code: Option<String>,
    ///Optional, stock keeping unit to link the item to other inventory systems.
    pub external_sku: Option<String>,
    ///This name describes your item and will appear on the invoice when it's purchased on a one time basis.
    pub name: String,
    pub revenue_schedule_type: Option<String>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the item is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types.
    pub avalara_transaction_type: Option<i64>,
    ///`true` exempts tax on the item, `false` applies tax on the item.
    pub tax_exempt: Option<bool>,
}
impl std::fmt::Display for ItemCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AccountReadOnly {
    ///When the account was last changed.
    pub updated_at: Option<String>,
    pub id: Option<String>,
    ///The unique token for automatically logging the account in to the hosted management pages. You may automatically log the user into their hosted management pages by directing the user to: `https://{subdomain}.recurly.com/account/{hosted_login_token}`.
    pub hosted_login_token: Option<String>,
    ///Accounts can be either active or inactive.
    pub state: Option<String>,
    ///Indicates if the account has an active subscription.
    pub has_active_subscription: Option<bool>,
    ///When the account was created.
    pub created_at: Option<String>,
    ///The shipping addresses on the account.
    pub shipping_addresses: Option<Vec<ShippingAddress>>,
    pub object: Option<String>,
    ///Indicates if the account has a paused subscription.
    pub has_paused_subscription: Option<bool>,
    ///Indicates if the account has a past due invoice.
    pub has_past_due_invoice: Option<bool>,
    ///If present, when the account was last marked inactive.
    pub deleted_at: Option<String>,
    ///Indicates if the account has a future subscription.
    pub has_future_subscription: Option<bool>,
    ///Indicates if the account has a canceled subscription.
    pub has_canceled_subscription: Option<bool>,
    ///Indicates if the account has a subscription that is either active, canceled, future, or paused.
    pub has_live_subscription: Option<bool>,
}
impl std::fmt::Display for AccountReadOnly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ItemList {
    pub data: Option<Vec<Item>>,
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
    ///Will always be List.
    pub object: Option<String>,
}
impl std::fmt::Display for ItemList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayment {
    pub object: Option<String>,
    ///For credit payments with action `refund`, this is the credit payment that was refunded.
    pub original_credit_payment_id: Option<String>,
    pub updated_at: Option<String>,
    pub id: Option<String>,
    ///The UUID is useful for matching data with the CSV exports and building URLs into Recurly's UI.
    pub uuid: Option<String>,
    pub applied_to_invoice: Option<InvoiceMini>,
    pub created_at: Option<String>,
    pub voided_at: Option<String>,
    pub account: Option<AccountMini>,
    ///3-letter ISO 4217 currency code.
    pub currency: Option<String>,
    ///The action for which the credit was created.
    pub action: Option<String>,
    pub original_invoice: Option<InvoiceMini>,
    ///Total credit payment amount applied to the charge invoice.
    pub amount: Option<f64>,
    pub refund_transaction: Option<Transaction>,
}
impl std::fmt::Display for CreditPayment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BillingInfoList {
    ///Will always be List.
    pub object: Option<String>,
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    pub data: Option<Vec<BillingInfo>>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
}
impl std::fmt::Display for BillingInfoList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LineItemRefund {
    pub id: Option<String>,
    ///Line item quantity to be refunded.
    pub quantity: Option<i64>,
    ///A floating-point alternative to Quantity. If this value is present, it will be used in place of Quantity for calculations, and Quantity will be the rounded integer value of this number. This field supports up to 9 decimal places. The Decimal Quantity feature must be enabled to utilize this field.
    pub quantity_decimal: Option<String>,
    /**Set to `true` if the line item should be prorated; set to `false` if not.
This can only be used on line items that have a start and end date.
*/
    pub prorate: Option<bool>,
}
impl std::fmt::Display for LineItemRefund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AddOnPricing {
    ///Allows up to 2 decimal places. Required unless `unit_amount_decimal` is provided.
    pub unit_amount: Option<f64>,
    ///3-letter ISO 4217 currency code.
    pub currency: String,
    ///This field is deprecated. Please do not use it.
    pub tax_inclusive: Option<bool>,
    /**Allows up to 9 decimal places. Only supported when `add_on_type` = `usage`.
If `unit_amount_decimal` is provided, `unit_amount` cannot be provided.
*/
    pub unit_amount_decimal: Option<String>,
}
impl std::fmt::Display for AddOnPricing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShippingMethodCreate {
    ///Accounting code for shipping method.
    pub accounting_code: Option<String>,
    /**Used by Avalara, Vertex, and Recurly’s built-in tax feature. The tax
code values are specific to each tax system. If you are using Recurly’s
built-in taxes the values are:

- `FR` – Common Carrier FOB Destination
- `FR022000` – Common Carrier FOB Origin
- `FR020400` – Non Common Carrier FOB Destination
- `FR020500` – Non Common Carrier FOB Origin
- `FR010100` – Delivery by Company Vehicle Before Passage of Title
- `FR010200` – Delivery by Company Vehicle After Passage of Title
- `NT` – Non-Taxable
*/
    pub tax_code: Option<String>,
    ///The internal name used identify the shipping method.
    pub code: String,
    ///The name of the shipping method displayed to customers.
    pub name: String,
}
impl std::fmt::Display for ShippingMethodCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalSubscription {
    ///When the external subscription was activated in the external platform.
    pub activated_at: Option<String>,
    ///External Product Reference details
    pub external_product_reference: Option<ExternalProductReferenceMini>,
    pub external_resource: Option<ExternalResourceMini>,
    pub account: Option<AccountMini>,
    ///When the external subscription expires in the external platform.
    pub expires_at: Option<String>,
    ///Identifier of the app that generated the external subscription.
    pub app_identifier: Option<String>,
    ///When a new billing event occurred on the external subscription in conjunction with a recent billing period, reactivation or upgrade/downgrade.
    pub last_purchased: Option<String>,
    ///When the external subscription was updated in Recurly.
    pub updated_at: Option<String>,
    ///An indication of the quantity of a subscribed item's quantity.
    pub quantity: Option<i64>,
    ///When the external subscription was created in Recurly.
    pub created_at: Option<String>,
    ///An indication of whether or not the external subscription will auto-renew at the expiration date.
    pub auto_renew: Option<bool>,
    ///System-generated unique identifier for an external subscription ID, e.g. `e28zov4fw0v2`.
    pub id: Option<String>,
    pub object: Option<String>,
}
impl std::fmt::Display for ExternalSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShippingFeeCreate {
    ///This is priced in the purchase's currency.
    pub amount: Option<f64>,
    ///The id of the shipping method used to deliver the purchase. If `method_id` and `method_code` are both present, `method_id` will be used.
    pub method_id: Option<String>,
    ///The code of the shipping method used to deliver the purchase. If `method_id` and `method_code` are both present, `method_id` will be used.
    pub method_code: Option<String>,
}
impl std::fmt::Display for ShippingFeeCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Entitlements {
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
    pub data: Option<Vec<Entitlement>>,
    pub object: Option<String>,
}
impl std::fmt::Display for Entitlements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExportFiles {
    pub object: Option<String>,
    pub files: Option<Vec<ExportFile>>,
}
impl std::fmt::Display for ExportFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MeasuredUnitList {
    ///Path to subsequent page of results.
    pub next: Option<String>,
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    ///Will always be List.
    pub object: Option<String>,
    pub data: Option<Vec<MeasuredUnit>>,
}
impl std::fmt::Display for MeasuredUnitList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShippingMethod {
    pub id: Option<String>,
    ///The internal name used identify the shipping method.
    pub code: Option<String>,
    pub updated_at: Option<String>,
    pub object: Option<String>,
    pub created_at: Option<String>,
    pub deleted_at: Option<String>,
    ///Accounting code for shipping method.
    pub accounting_code: Option<String>,
    ///The name of the shipping method displayed to customers.
    pub name: Option<String>,
    /**Used by Avalara, Vertex, and Recurly’s built-in tax feature. The tax
code values are specific to each tax system. If you are using Recurly’s
built-in taxes the values are:

- `FR` – Common Carrier FOB Destination
- `FR022000` – Common Carrier FOB Origin
- `FR020400` – Non Common Carrier FOB Destination
- `FR020500` – Non Common Carrier FOB Origin
- `FR010100` – Delivery by Company Vehicle Before Passage of Title
- `FR010200` – Delivery by Company Vehicle After Passage of Title
- `NT` – Non-Taxable
*/
    pub tax_code: Option<String>,
}
impl std::fmt::Display for ShippingMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubscriptionShippingPurchase {
    ///The code of the shipping method used to deliver the subscription. If `method_id` and `method_code` are both present, `method_id` will be used.
    pub method_code: Option<String>,
    ///The id of the shipping method used to deliver the subscription. If `method_id` and `method_code` are both present, `method_id` will be used.
    pub method_id: Option<String>,
    pub amount: Option<f64>,
}
impl std::fmt::Display for SubscriptionShippingPurchase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceRefund {
    /**Used as the Customer Notes on the credit invoice.

This field can only be include when the Credit Invoices feature is enabled.
*/
    pub credit_customer_notes: Option<String>,
    /**The amount to be refunded. The amount will be split between the line items.
If no amount is specified, it will default to refunding the total refundable amount on the invoice.
*/
    pub amount: Option<f64>,
    /**Indicates that the refund was settled outside of Recurly, and a manual transaction should be created to track it in Recurly.

Required when:
- refunding a manually collected charge invoice, and `refund_method` is not `all_credit`
- refunding a credit invoice that refunded manually collecting invoices
- refunding a credit invoice for a partial amount

This field can only be included when the Credit Invoices feature is enabled.
*/
    pub external_refund: Option<serde_json::Value>,
    ///The line items to be refunded. This is required when `type=line_items`.
    pub line_items: Option<Vec<LineItemRefund>>,
    /**Indicates how the invoice should be refunded when both a credit and transaction are present on the invoice:
- `transaction_first` – Refunds the transaction first, then any amount is issued as credit back to the account. Default value when Credit Invoices feature is enabled.
- `credit_first` – Issues credit back to the account first, then refunds any remaining amount back to the transaction. Default value when Credit Invoices feature is not enabled.
- `all_credit` – Issues credit to the account for the entire amount of the refund. Only available when the Credit Invoices feature is enabled.
- `all_transaction` – Refunds the entire amount back to transactions, using transactions from previous invoices if necessary. Only available when the Credit Invoices feature is enabled.
*/
    pub refund_method: Option<String>,
    ///The type of refund. Amount and line items cannot both be specified in the request.
    pub type_: String,
}
impl std::fmt::Display for InvoiceRefund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubscriptionCancel {
    ///The timeframe parameter controls when the expiration takes place. The `bill_date` timeframe causes the subscription to expire when the subscription is scheduled to bill next. The `term_end` timeframe causes the subscription to continue to bill until the end of the subscription term, then expire.
    pub timeframe: Option<String>,
}
impl std::fmt::Display for SubscriptionCancel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AddOn {
    ///The unique identifier for the add-on within its plan.
    pub code: String,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the add-on is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types.
    pub avalara_service_type: Option<i64>,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the add-on is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types.
    pub avalara_transaction_type: Option<i64>,
    pub id: Option<String>,
    ///Type of usage, returns usage type if `add_on_type` is `usage`.
    pub usage_type: Option<String>,
    ///Accounting code for invoice line items for this add-on. If no value is provided, it defaults to add-on's code.
    pub accounting_code: Option<String>,
    ///Used by Avalara, Vertex, and Recurly’s EU VAT tax feature. The tax code values are specific to each tax system. If you are using Recurly’s EU VAT feature you can use `unknown`, `physical`, or `digital`.
    pub tax_code: Option<String>,
    pub object: Option<String>,
    ///The type of calculation to be employed for an add-on.  Cumulative billing will sum all usage records created in the current billing cycle.  Last-in-period billing will apply only the most recent usage record in the billing period.  If no value is specified, cumulative billing will be used.
    pub usage_calculation_type: Option<String>,
    ///Whether the add-on type is fixed, or usage-based.
    pub add_on_type: Option<String>,
    ///Default quantity for the hosted pages.
    pub default_quantity: Option<i64>,
    ///Just the important parts.
    pub item: Option<ItemMini>,
    pub currencies: Option<Vec<AddOnPricing>>,
    ///The time at which usage totals are reset for billing purposes.
    pub usage_timeframe: Option<String>,
    ///System-generated unique identifier for an measured unit associated with the add-on.
    pub measured_unit_id: Option<String>,
    pub tiers: Option<Vec<Tier>>,
    pub percentage_tiers: Option<Vec<PercentageTiersByCurrency>>,
    ///When this add-on is invoiced, the line item will use this revenue schedule. If `item_code`/`item_id` is part of the request then `revenue_schedule_type` must be absent in the request as the value will be set from the item.
    pub revenue_schedule_type: Option<String>,
    ///Whether the add-on is optional for the customer to include in their purchase on the hosted payment page. If false, the add-on will be included when a subscription is created through the Recurly UI. However, the add-on will not be included when a subscription is created through the API.
    pub optional: Option<bool>,
    pub plan_id: Option<String>,
    /**The pricing model for the add-on.  For more information,
[click here](https://docs.recurly.com/docs/billing-models#section-quantity-based). See our
[Guide](https://recurly.com/developers/guides/item-addon-guide.html) for an overview of how
to configure quantity-based pricing models.
*/
    pub tier_type: Option<String>,
    pub deleted_at: Option<String>,
    ///Describes your add-on and will appear in subscribers' invoices.
    pub name: String,
    ///Add-ons can be either active or inactive.
    pub state: Option<String>,
    ///Optional, stock keeping unit to link the item to other inventory systems.
    pub external_sku: Option<String>,
    ///Determines if the quantity field is displayed on the hosted pages for the add-on.
    pub display_quantity: Option<bool>,
    ///The percentage taken of the monetary amount of usage tracked. This can be up to 4 decimal places. A value between 0.0 and 100.0.
    pub usage_percentage: Option<f64>,
}
impl std::fmt::Display for AddOn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GrantedBy {
    pub object: Option<String>,
    ///The ID of the subscription or external subscription that grants the permission to the account.
    pub id: Option<String>,
}
impl std::fmt::Display for GrantedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub currencies: Option<Vec<Pricing>>,
    ///Optional, stock keeping unit to link the item to other inventory systems.
    pub external_sku: Option<String>,
    ///This name describes your item and will appear on the invoice when it's purchased on a one time basis.
    pub name: Option<String>,
    pub id: Option<String>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the item is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types.
    pub avalara_transaction_type: Option<i64>,
    pub object: Option<String>,
    ///The current state of the item.
    pub state: Option<String>,
    pub revenue_schedule_type: Option<String>,
    pub created_at: Option<String>,
    ///Unique code to identify the item.
    pub code: Option<String>,
    ///Optional, description.
    pub description: Option<String>,
    ///The custom fields will only be altered when they are included in a request. Sending an empty array will not remove any existing values. To remove a field send the name with a null or empty value.
    pub custom_fields: Option<CustomFields>,
    ///`true` exempts tax on the item, `false` applies tax on the item.
    pub tax_exempt: Option<bool>,
    pub updated_at: Option<String>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the item is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types.
    pub avalara_service_type: Option<i64>,
    pub deleted_at: Option<String>,
    ///Used by Avalara, Vertex, and Recurly’s EU VAT tax feature. The tax code values are specific to each tax system. If you are using Recurly’s EU VAT feature you can use `unknown`, `physical`, or `digital`.
    pub tax_code: Option<String>,
    ///Accounting code for invoice line items.
    pub accounting_code: Option<String>,
}
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AddOnCreate {
    ///Default quantity for the hosted pages.
    pub default_quantity: Option<i64>,
    ///Accounting code for invoice line items for this add-on. If no value is provided, it defaults to add-on's code. If `item_code`/`item_id` is part of the request then `accounting_code` must be absent.
    pub accounting_code: Option<String>,
    ///Whether the add-on is optional for the customer to include in their purchase on the hosted payment page. If false, the add-on will be included when a subscription is created through the Recurly UI. However, the add-on will not be included when a subscription is created through the API.
    pub optional: Option<bool>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the add-on is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types. If an `Item` is associated to the `AddOn`, then the `avalara_transaction_type` must be absent.
    pub avalara_transaction_type: Option<i64>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the add-on is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types. If an `Item` is associated to the `AddOn`, then the `avalara_service_type` must be absent.
    pub avalara_service_type: Option<i64>,
    ///Whether the add-on type is fixed, or usage-based.
    pub add_on_type: Option<String>,
    ///System-generated unique identifier for an item. Available when the `Credit Invoices` feature is enabled. If `item_id` and `item_code` are both present, `item_id` will be used.
    pub item_id: Option<String>,
    ///Optional field used by Avalara, Vertex, and Recurly's EU VAT tax feature to determine taxation rules. If you have your own AvaTax or Vertex account configured, use their tax codes to assign specific tax rules. If you are using Recurly's EU VAT feature, you can use values of `unknown`, `physical`, or `digital`. If `item_code`/`item_id` is part of the request then `tax_code` must be absent.
    pub tax_code: Option<String>,
    ///Name of a measured unit to be associated with the add-on. Either `measured_unit_id` or `measured_unit_name` are required when `add_on_type` is `usage`. If `measured_unit_id` and `measured_unit_name` are both present, `measured_unit_id` will be used.
    pub measured_unit_name: Option<String>,
    ///Unique code to identify an item. Available when the `Credit Invoices` feature are enabled. If `item_id` and `item_code` are both present, `item_id` will be used.
    pub item_code: Option<String>,
    #[doc = "* If `item_code`/`item_id` is part of the request and the item\nhas a default currency then `currencies` is optional. If the item does\nnot have a default currency, then `currencies` is required. If `item_code`/`item_id`\nis not present `currencies` is required.\n* If the add-on's `tier_type` is `tiered`, `volume`, or `stairstep`,\nthen `currencies` must be absent.\n* Must be absent if `add_on_type` is `usage` and `usage_type` is `percentage`.\n"]
    pub currencies: Option<Vec<AddOnPricing>>,
    pub plan_id: Option<String>,
    /**The pricing model for the add-on.  For more information,
[click here](https://docs.recurly.com/docs/billing-models#section-quantity-based). See our
[Guide](https://recurly.com/developers/guides/item-addon-guide.html) for an overview of how
to configure quantity-based pricing models.
*/
    pub tier_type: Option<String>,
    /**If the tier_type is `flat`, then `tiers` must be absent. The `tiers` object
must include one to many tiers with `ending_quantity` and `unit_amount` for
the desired `currencies`. There must be one tier without an `ending_quantity` value
which represents the final tier.
*/
    pub tiers: Option<Vec<Tier>>,
    /**The time at which usage totals are reset for billing purposes.
Allows for `tiered` add-ons to accumulate usage over the course of multiple
billing periods.
*/
    pub usage_timeframe: Option<String>,
    /**Type of usage, required if `add_on_type` is `usage`. See our
[Guide](https://recurly.com/developers/guides/usage-based-billing-guide.html) for an
overview of how to configure usage add-ons.
*/
    pub usage_type: Option<String>,
    ///When this add-on is invoiced, the line item will use this revenue schedule. If `item_code`/`item_id` is part of the request then `revenue_schedule_type` must be absent in the request as the value will be set from the item.
    pub revenue_schedule_type: Option<String>,
    ///The percentage taken of the monetary amount of usage tracked. This can be up to 4 decimal places. A value between 0.0 and 100.0. Required if `add_on_type` is usage, `tier_type` is `flat` and `usage_type` is percentage. Must be omitted otherwise.
    pub usage_percentage: Option<f64>,
    ///Determines if the quantity field is displayed on the hosted pages for the add-on.
    pub display_quantity: Option<bool>,
    ///The unique identifier for the add-on within its plan. If `item_code`/`item_id` is part of the request then `code` must be absent. If `item_code`/`item_id` is not present `code` is required.
    pub code: String,
    ///Describes your add-on and will appear in subscribers' invoices. If `item_code`/`item_id` is part of the request then `name` must be absent. If `item_code`/`item_id` is not present `name` is required.
    pub name: String,
    /**Array of objects which must have at least one set of tiers
per currency and the currency code. The tier_type must be `volume` or `tiered`,
if not, it must be absent. There must be one tier without an `ending_amount` value
which represents the final tier.
*/
    pub percentage_tiers: Option<Vec<PercentageTiersByCurrency>>,
    ///System-generated unique identifier for a measured unit to be associated with the add-on. Either `measured_unit_id` or `measured_unit_name` are required when `add_on_type` is `usage`. If `measured_unit_id` and `measured_unit_name` are both present, `measured_unit_id` will be used.
    pub measured_unit_id: Option<String>,
    ///The type of calculation to be employed for an add-on.  Cumulative billing will sum all usage records created in the current billing cycle.  Last-in-period billing will apply only the most recent usage record in the billing period.  If no value is specified, cumulative billing will be used.
    pub usage_calculation_type: Option<String>,
}
impl std::fmt::Display for AddOnCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LineItemCreate {
    ///Optional field used by Avalara, Vertex, and Recurly's EU VAT tax feature to determine taxation rules. If you have your own AvaTax or Vertex account configured, use their tax codes to assign specific tax rules. If you are using Recurly's EU VAT feature, you can use values of `unknown`, `physical`, or `digital`.
    pub tax_code: Option<String>,
    /**A positive or negative amount with `type=charge` will result in a positive `unit_amount`.
A positive or negative amount with `type=credit` will result in a negative `unit_amount`.
If `item_code`/`item_id` is present, `unit_amount` can be passed in, to override the
`Item`'s `unit_amount`. If `item_code`/`item_id` is not present then `unit_amount` is required.
*/
    pub unit_amount: f64,
    ///System-generated unique identifier for an item. Available when the Credit Invoices feature is enabled.
    pub item_id: Option<String>,
    pub revenue_schedule_type: Option<String>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the line item is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types. If an `Item` is associated to the `LineItem`, then the `avalara_service_type` must be absent.
    pub avalara_service_type: Option<i64>,
    ///This number will be multiplied by the unit amount to compute the subtotal before any discounts or taxes.
    pub quantity: Option<i64>,
    ///Determines whether or not tax is included in the unit amount. The Tax Inclusive Pricing feature (separate from the Mixed Tax Pricing feature) must be enabled to use this flag.
    pub tax_inclusive: Option<bool>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the line item is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types. If an `Item` is associated to the `LineItem`, then the `avalara_transaction_type` must be absent.
    pub avalara_transaction_type: Option<i64>,
    ///If an end date is present, this is value indicates the beginning of a billing time range. If no end date is present it indicates billing for a specific date. Defaults to the current date-time.
    pub start_date: Option<String>,
    ///If this date is provided, it indicates the end of a time range.
    pub end_date: Option<String>,
    ///`true` exempts tax on charges, `false` applies tax on charges. If not defined, then defaults to the Plan and Site settings. This attribute does not work for credits (negative line items). Credits are always applied post-tax. Pre-tax discounts should use the Coupons feature.
    pub tax_exempt: Option<bool>,
    ///Origin `external_gift_card` is allowed if the Gift Cards feature is enabled on your site and `type` is `credit`. Set this value in order to track gift card credits from external gift cards (like InComm). It also skips billing information requirements.  Origin `prepayment` is only allowed if `type` is `charge` and `tax_exempt` is left blank or set to true.  This origin creates a charge and opposite credit on the account to be used for future invoices.
    pub origin: Option<String>,
    ///3-letter ISO 4217 currency code. If `item_code`/`item_id` is part of the request then `currency` is optional, if the site has a single default currency. `currency` is required if `item_code`/`item_id` is present, and there are multiple currencies defined on the site. If `item_code`/`item_id` is not present `currency` is required.
    pub currency: String,
    ///Description that appears on the invoice. If `item_code`/`item_id` is part of the request then `description` must be absent.
    pub description: Option<String>,
    ///Line item type. If `item_code`/`item_id` is present then `type` should not be present. If `item_code`/`item_id` is not present then `type` is required.
    pub type_: String,
    ///The reason the credit was given when line item is `type=credit`. When the Credit Invoices feature is enabled, the value can be set and will default to `general`. When the Credit Invoices feature is not enabled, the value will always be `null`.
    pub credit_reason_code: Option<String>,
    ///Accounting Code for the `LineItem`. If `item_code`/`item_id` is part of the request then `accounting_code` must be absent.
    pub accounting_code: Option<String>,
    ///Unique code to identify an item. Available when the Credit Invoices feature is enabled.
    pub item_code: Option<String>,
    ///Optional field to track a product code or SKU for the line item. This can be used to later reporting on product purchases. For Vertex tax calculations, this field will be used as the Vertex `product` field. If `item_code`/`item_id` is part of the request then `product_code` must be absent.
    pub product_code: Option<String>,
}
impl std::fmt::Display for LineItemCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountPurchase {
    pub account_update: AccountUpdate,
    ///Optional, but if present allows an existing account to be used and updated as part of the purchase.
    pub id: String,
    pub acquisition: AccountAcquisitionUpdate,
    ///The unique identifier of the account. This cannot be changed once the account is created.
    pub code: String,
}
impl std::fmt::Display for AccountPurchase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DunningCampaignsBulkUpdateResponse(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AccountMini {
    pub bill_to: Option<String>,
    ///The unique identifier of the account.
    pub code: Option<String>,
    pub first_name: Option<String>,
    pub company: Option<String>,
    pub id: Option<String>,
    pub parent_account_id: Option<String>,
    ///Unique ID to identify a dunning campaign. Used to specify if a non-default dunning campaign should be assigned to this account. For sites without multiple dunning campaigns enabled, the default dunning campaign will always be used.
    pub dunning_campaign_id: Option<String>,
    pub object: Option<String>,
    pub last_name: Option<String>,
    ///The email address used for communicating with this customer.
    pub email: Option<String>,
}
impl std::fmt::Display for AccountMini {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InvoiceMini {
    pub number: Option<String>,
    pub type_: Option<String>,
    pub object: Option<String>,
    pub state: Option<String>,
    pub id: Option<String>,
}
impl std::fmt::Display for InvoiceMini {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlanMini {
    ///This name describes your plan and will appear on the Hosted Payment Page and the subscriber's invoice.
    pub name: Option<String>,
    ///Unique code to identify the plan. This is used in Hosted Payment Page URLs and in the invoice exports.
    pub code: Option<String>,
    pub id: Option<String>,
    pub object: Option<String>,
}
impl std::fmt::Display for PlanMini {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreditPaymentList {
    pub data: Option<Vec<CreditPayment>>,
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    ///Will always be List.
    pub object: Option<String>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
}
impl std::fmt::Display for CreditPaymentList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlanRampPricing {
    ///3-letter ISO 4217 currency code.
    pub currency: String,
    ///Represents the price for the Ramp Interval.
    pub unit_amount: f64,
}
impl std::fmt::Display for PlanRampPricing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InvoiceTemplateList {
    pub data: Option<Vec<InvoiceTemplate>>,
    ///Will always be List.
    pub object: Option<String>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
}
impl std::fmt::Display for InvoiceTemplateList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DunningCampaignList {
    ///Path to subsequent page of results.
    pub next: Option<String>,
    ///Will always be List.
    pub object: Option<String>,
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    pub data: Option<Vec<DunningCampaign>>,
}
impl std::fmt::Display for DunningCampaignList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CouponUpdate {
    ///The date and time the coupon will expire and can no longer be redeemed. Time is always 11:59:59, the end-of-day Pacific time.
    pub redeem_by_date: Option<String>,
    ///A maximum number of redemptions for the coupon. The coupon will expire when it hits its maximum redemptions.
    pub max_redemptions: Option<i64>,
    ///This description will show up when a customer redeems a coupon on your Hosted Payment Pages, or if you choose to show the description on your own checkout page.
    pub hosted_description: Option<String>,
    ///The internal name for the coupon.
    pub name: Option<String>,
    ///Redemptions per account is the number of times a specific account can redeem the coupon. Set redemptions per account to `1` if you want to keep customers from gaming the system and getting more than one discount from the coupon campaign.
    pub max_redemptions_per_account: Option<i64>,
    ///Description of the coupon on the invoice.
    pub invoice_description: Option<String>,
}
impl std::fmt::Display for CouponUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionChange {
    ///The custom fields will only be altered when they are included in a request. Sending an empty array will not remove any existing values. To remove a field send the name with a null or empty value.
    pub custom_fields: Option<CustomFields>,
    pub quantity: Option<i64>,
    pub shipping: Option<SubscriptionShipping>,
    ///This field is deprecated. Please do not use it.
    pub tax_inclusive: Option<bool>,
    pub revenue_schedule_type: Option<String>,
    ///The ramp intervals representing the pricing schedule for the subscription.
    pub ramp_intervals: Option<Vec<SubscriptionRampIntervalResponse>>,
    ///The ID of the Subscription Change.
    pub id: Option<String>,
    ///The ID of the subscription that is going to be changed.
    pub subscription_id: Option<String>,
    pub activate_at: Option<String>,
    pub updated_at: Option<String>,
    ///Just the important parts.
    pub plan: Option<PlanMini>,
    ///Returns `true` if the subscription change is activated.
    pub activated: Option<bool>,
    pub unit_amount: Option<f64>,
    ///Accept nested attributes for three_d_secure_action_result_token_id
    pub billing_info: Option<SubscriptionChangeBillingInfo>,
    ///These add-ons will be used when the subscription renews.
    pub add_ons: Option<Vec<SubscriptionAddOn>>,
    pub object: Option<String>,
    pub deleted_at: Option<String>,
    pub created_at: Option<String>,
    pub invoice_collection: Option<InvoiceCollection>,
}
impl std::fmt::Display for SubscriptionChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExternalProductReferenceMini {
    ///Source connection platform.
    pub external_connection_type: Option<String>,
    pub object: Option<String>,
    ///A code which associates the external product to a corresponding object or resource in an external platform like the Apple App Store or Google Play Store.
    pub reference_code: Option<String>,
    ///When the external product was updated in Recurly.
    pub updated_at: Option<String>,
    ///System-generated unique identifier for an external product ID, e.g. `e28zov4fw0v2`.
    pub id: Option<String>,
    ///When the external product was created in Recurly.
    pub created_at: Option<String>,
}
impl std::fmt::Display for ExternalProductReferenceMini {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMayHaveTransaction {
    pub error: Error,
    ///This is only included on errors with `type=transaction`.
    pub transaction_error: serde_json::Value,
}
impl std::fmt::Display for ErrorMayHaveTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TaxInfo {
    pub rate: Option<f64>,
    ///Provides additional tax details for Canadian Sales Tax when there is tax applied at both the country and province levels. This will only be populated for the Invoice response when fetching a single invoice and not for the InvoiceList or LineItem.
    pub tax_details: Option<Vec<TaxDetail>>,
    ///Provides the tax type as "vat" for EU VAT, "usst" for U.S. Sales Tax, or the 2 letter country code for country level tax types like Canada, Australia, New Zealand, Israel, and all non-EU European countries.
    pub type_: Option<String>,
    ///Provides the tax region applied on an invoice. For U.S. Sales Tax, this will be the 2 letter state code. For EU VAT this will be the 2 letter country code. For all country level tax types, this will display the regional tax, like VAT, GST, or PST.
    pub region: Option<String>,
}
impl std::fmt::Display for TaxInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CouponList {
    ///Will always be List.
    pub object: Option<String>,
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    pub data: Option<Vec<Coupon>>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
}
impl std::fmt::Display for CouponList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomFieldDefinition {
    pub id: Option<String>,
    ///Definitions are initially soft deleted, and once all the values are removed from the accouts or subscriptions, will be hard deleted an no longer visible.
    pub deleted_at: Option<String>,
    pub object: Option<String>,
    pub related_type: Option<String>,
    /**The access control applied inside Recurly's admin UI:
- `api_only` - No one will be able to view or edit this field's data via the admin UI.
- `read_only` - Users with the Customers role will be able to view this field's data via the admin UI, but
  editing will only be available via the API.
- `write` - Users with the Customers role will be able to view and edit this field's data via the admin UI.
*/
    pub user_access: Option<String>,
    pub created_at: Option<String>,
    ///Used to label the field when viewing and editing the field in Recurly's admin UI.
    pub display_name: Option<String>,
    pub updated_at: Option<String>,
    ///Used by the API to identify the field or reading and writing. The name can only be used once per Recurly object type.
    pub name: Option<String>,
    ///Displayed as a tooltip when editing the field in the Recurly admin UI.
    pub tooltip: Option<String>,
}
impl std::fmt::Display for CustomFieldDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PercentageTier {
    ///Ending amount for the tier. Allows up to 2 decimal places. Must be left empty if it is the final tier.
    pub ending_amount: Option<f64>,
    /**The percentage taken of the monetary amount of usage tracked.
This can be up to 4 decimal places represented as a string.
*/
    pub usage_percentage: Option<String>,
}
impl std::fmt::Display for PercentageTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DunningCampaign {
    ///Whether or not this is the default campaign for accounts or plans without an assigned dunning campaign.
    pub default_campaign: Option<bool>,
    ///Campaign description.
    pub description: Option<String>,
    ///Campaign name.
    pub name: Option<String>,
    ///Dunning Cycle settings.
    pub dunning_cycles: Option<Vec<DunningCycle>>,
    ///When the current campaign was created in Recurly.
    pub created_at: Option<String>,
    pub id: Option<String>,
    pub object: Option<String>,
    ///When the current campaign was updated in Recurly.
    pub updated_at: Option<String>,
    ///When the current campaign was deleted in Recurly.
    pub deleted_at: Option<String>,
    ///Campaign code.
    pub code: Option<String>,
}
impl std::fmt::Display for DunningCampaign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Coupon {
    /**Details of the discount a coupon applies. Will contain a `type`
property and one of the following properties: `percent`, `fixed`, `trial`.
*/
    pub discount: Option<CouponDiscount>,
    /**- "single_use" coupons applies to the first invoice only.
- "temporal" coupons will apply to invoices for the duration determined by the `temporal_unit` and `temporal_amount` attributes.
*/
    pub duration: Option<String>,
    ///Description of the coupon on the invoice.
    pub invoice_description: Option<String>,
    ///Will be populated when the Coupon being returned is a `UniqueCouponCode`.
    pub unique_coupon_code: Option<serde_json::Value>,
    ///The coupon is valid for one-time, non-plan charges if true.
    pub applies_to_non_plan_charges: Option<bool>,
    ///The date and time the coupon will expire and can no longer be redeemed. Time is always 11:59:59, the end-of-day Pacific time.
    pub redeem_by: Option<String>,
    ///The date and time the coupon was expired early or reached its `max_redemptions`.
    pub expired_at: Option<String>,
    ///A list of plans for which this coupon applies. This will be `null` if `applies_to_all_plans=true`.
    pub plans: Option<Vec<PlanMini>>,
    ///Description of the unit of time the coupon is for. Used with `free_trial_amount` to determine the duration of time the coupon is for.
    pub free_trial_unit: Option<String>,
    ///Indicates if the coupon is redeemable, and if it is not, why.
    pub state: Option<String>,
    ///The internal name for the coupon.
    pub name: Option<String>,
    ///The coupon is valid for all plans if true. If false then `plans` will list the applicable plans.
    pub applies_to_all_plans: Option<bool>,
    pub object: Option<String>,
    /**The coupon is valid for all items if true. If false then `items`
will list the applicable items.
*/
    pub applies_to_all_items: Option<bool>,
    ///The code the customer enters to redeem the coupon.
    pub code: Option<String>,
    ///Redemptions per account is the number of times a specific account can redeem the coupon. Set redemptions per account to `1` if you want to keep customers from gaming the system and getting more than one discount from the coupon campaign.
    pub max_redemptions_per_account: Option<i64>,
    ///If `duration` is "temporal" than `temporal_unit` is multiplied by `temporal_amount` to define the duration that the coupon will be applied to invoices for.
    pub temporal_unit: Option<String>,
    pub id: Option<String>,
    pub updated_at: Option<String>,
    ///Sets the duration of time the `free_trial_unit` is for.
    pub free_trial_amount: Option<i64>,
    /**A list of items for which this coupon applies. This will be
`null` if `applies_to_all_items=true`.
*/
    pub items: Option<Vec<ItemMini>>,
    ///Whether the discount is for all eligible charges on the account, or only a specific subscription.
    pub redemption_resource: Option<String>,
    ///Whether the coupon is "single_code" or "bulk". Bulk coupons will require a `unique_code_template` and will generate unique codes through the `/generate` endpoint.
    pub coupon_type: Option<String>,
    pub created_at: Option<String>,
    ///When this number reaches `max_redemptions` the coupon will no longer be redeemable.
    pub unique_coupon_codes_count: Option<i64>,
    ///A maximum number of redemptions for the coupon. The coupon will expire when it hits its maximum redemptions.
    pub max_redemptions: Option<i64>,
    ///This description will show up when a customer redeems a coupon on your Hosted Payment Pages, or if you choose to show the description on your own checkout page.
    pub hosted_page_description: Option<String>,
    ///On a bulk coupon, the template from which unique coupon codes are generated.
    pub unique_code_template: Option<String>,
    ///If `duration` is "temporal" than `temporal_amount` is an integer which is multiplied by `temporal_unit` to define the duration that the coupon will be applied to invoices for.
    pub temporal_amount: Option<i64>,
}
impl std::fmt::Display for Coupon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalProduct {
    ///Name to identify the external product in Recurly.
    pub name: Option<String>,
    ///System-generated unique identifier for an external product ID, e.g. `e28zov4fw0v2`.
    pub id: Option<String>,
    pub object: Option<String>,
    ///When the external product was updated in Recurly.
    pub updated_at: Option<String>,
    ///Just the important parts.
    pub plan: Option<PlanMini>,
    ///When the external product was created in Recurly.
    pub created_at: Option<String>,
    ///List of external product references of the external product.
    pub external_product_references: Option<Vec<ExternalProductReferenceMini>>,
}
impl std::fmt::Display for ExternalProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SiteList {
    ///Will always be List.
    pub object: Option<String>,
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    pub data: Option<Vec<Site>>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
}
impl std::fmt::Display for SiteList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BillingInfo {
    pub valid: Option<bool>,
    pub payment_method: Option<PaymentMethod>,
    ///The `primary_payment_method` field is used to indicate the primary billing info on the account. The first billing info created on an account will always become primary. This payment method will be used
    pub primary_payment_method: Option<bool>,
    pub last_name: Option<String>,
    ///When the billing information was last changed.
    pub updated_at: Option<String>,
    pub first_name: Option<String>,
    ///When the billing information was created.
    pub created_at: Option<String>,
    ///Customer's VAT number (to avoid having the VAT applied). This is only used for automatically collected invoices.
    pub vat_number: Option<String>,
    pub address: Option<Address>,
    ///Most recent fraud result.
    pub fraud: Option<serde_json::Value>,
    ///The `backup_payment_method` field is used to indicate a billing info as a backup on the account that will be tried if the initial billing info used for an invoice is declined.
    pub backup_payment_method: Option<bool>,
    pub object: Option<String>,
    pub account_id: Option<String>,
    pub updated_by: Option<serde_json::Value>,
    pub company: Option<String>,
    pub id: Option<String>,
}
impl std::fmt::Display for BillingInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomField {
    ///Fields must be created in the UI before values can be assigned to them.
    pub name: String,
    ///Any values that resemble a credit card number or security code (CVV/CVC) will be rejected.
    pub value: String,
}
impl std::fmt::Display for CustomField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Invoice {
    pub object: Option<String>,
    pub address: Option<InvoiceAddress>,
    ///Invoices are either charge, credit, or legacy invoices.
    pub type_: Option<String>,
    pub state: Option<String>,
    ///Identifies if the invoice has more line items than are returned in `line_items`. If `has_more_line_items` is `true`, then a request needs to be made to the `list_invoice_line_items` endpoint.
    pub has_more_line_items: Option<bool>,
    ///If VAT taxation and the Country Invoice Sequencing feature are enabled, invoices will have country-specific invoice numbers for invoices billed to EU countries (ex: FR1001). Non-EU invoices will continue to use the site-level invoice number sequence.
    pub number: Option<String>,
    ///The `billing_info_id` is the value that represents a specific billing info for an end customer. When `billing_info_id` is used to assign billing info to the subscription, all future billing events for the subscription will bill to the specified billing info. `billing_info_id` can ONLY be used for sites utilizing the Wallet feature.
    pub billing_info_id: Option<String>,
    ///3-letter ISO 4217 currency code.
    pub currency: Option<String>,
    ///The total tax on this invoice.
    pub tax: Option<f64>,
    ///The summation of charges and credits, before discounts and taxes.
    pub subtotal: Option<f64>,
    ///Total discounts applied to this invoice.
    pub discount: Option<f64>,
    ///The refundable amount on a charge invoice. It will be null for all other invoices.
    pub refundable_amount: Option<f64>,
    ///Date invoice is due. This is the date the net terms are reached.
    pub due_at: Option<String>,
    pub line_items: Option<Vec<LineItem>>,
    pub transactions: Option<Vec<Transaction>>,
    ///Integer representing the number of days after an invoice's creation that the invoice will become past due. If an invoice's net terms are set to '0', it is due 'On Receipt' and will become past due 24 hours after it’s created. If an invoice is due net 30, it will become past due at 31 days exactly.
    pub net_terms: Option<i64>,
    ///This will default to the Customer Notes text specified on the Invoice Settings. Specify custom notes to add or override Customer Notes.
    pub customer_notes: Option<String>,
    pub credit_payments: Option<Vec<CreditPayment>>,
    ///On refund invoices, this value will exist and show the invoice ID of the purchase invoice the refund was created from.
    pub previous_invoice_id: Option<String>,
    ///Unique ID to identify the dunning campaign used when dunning the invoice. For sites without multiple dunning campaigns enabled, this will always be the default dunning campaign.
    pub dunning_campaign_id: Option<String>,
    pub updated_at: Option<String>,
    ///Last communication attempt.
    pub final_dunning_event: Option<bool>,
    pub uuid: Option<String>,
    ///An automatic invoice means a corresponding transaction is run using the account's billing information at the same time the invoice is created. Manual invoices are created without a corresponding transaction. The merchant must enter a manual payment transaction or have the customer pay the invoice with an automatic method, like credit card, PayPal, Amazon, or ACH bank payment.
    pub collection_method: Option<String>,
    ///If the invoice is charging or refunding for one or more subscriptions, these are their IDs.
    pub subscription_ids: Option<Vec<String>>,
    ///VAT registration number for the customer on this invoice. This will come from the VAT Number field in the Billing Info or the Account Info depending on your tax settings and the invoice collection method.
    pub vat_number: Option<String>,
    ///VAT Reverse Charge Notes only appear if you have EU VAT enabled or are using your own Avalara AvaTax account and the customer is in the EU, has a VAT number, and is in a different country than your own. This will default to the VAT Reverse Charge Notes text specified on the Tax Settings page in your Recurly admin, unless custom notes were created with the original subscription.
    pub vat_reverse_charge_notes: Option<String>,
    pub account: Option<AccountMini>,
    ///The final total on this invoice. The summation of invoice charges, discounts, credits, and tax.
    pub total: Option<f64>,
    pub id: Option<String>,
    ///The event that created the invoice.
    pub origin: Option<String>,
    pub tax_info: Option<TaxInfo>,
    pub shipping_address: Option<ShippingAddress>,
    ///Number of times the event was sent.
    pub dunning_events_sent: Option<i64>,
    ///The outstanding balance remaining on this invoice.
    pub balance: Option<f64>,
    ///This will default to the Terms and Conditions text specified on the Invoice Settings page in your Recurly admin. Specify custom notes to add or override Terms and Conditions.
    pub terms_and_conditions: Option<String>,
    ///For manual invoicing, this identifies the PO number associated with the subscription.
    pub po_number: Option<String>,
    ///The total amount of successful payments transaction on this invoice.
    pub paid: Option<f64>,
    pub created_at: Option<String>,
    ///Date invoice was marked paid or failed.
    pub closed_at: Option<String>,
}
impl std::fmt::Display for Invoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShippingMethodUpdate {
    ///The name of the shipping method displayed to customers.
    pub name: Option<String>,
    /**Used by Avalara, Vertex, and Recurly’s built-in tax feature. The tax
code values are specific to each tax system. If you are using Recurly’s
built-in taxes the values are:

- `FR` – Common Carrier FOB Destination
- `FR022000` – Common Carrier FOB Origin
- `FR020400` – Non Common Carrier FOB Destination
- `FR020500` – Non Common Carrier FOB Origin
- `FR010100` – Delivery by Company Vehicle Before Passage of Title
- `FR010200` – Delivery by Company Vehicle After Passage of Title
- `NT` – Non-Taxable
*/
    pub tax_code: Option<String>,
    ///The internal name used identify the shipping method.
    pub code: Option<String>,
    ///Accounting code for shipping method.
    pub accounting_code: Option<String>,
}
impl std::fmt::Display for ShippingMethodUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubscriptionRampIntervalResponse {
    ///Represents the price for the ramp interval.
    pub unit_amount: Option<f64>,
    ///Represents how many billing cycles are left in a ramp interval.
    pub remaining_billing_cycles: Option<i64>,
    ///Represents the billing cycle where a ramp interval starts.
    pub starting_billing_cycle: Option<i64>,
}
impl std::fmt::Display for SubscriptionRampIntervalResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionShippingUpdate {
    ///Assign a shipping address from the account's existing shipping addresses.
    pub address_id: Option<String>,
    pub object: Option<String>,
    pub address: Option<ShippingAddressCreate>,
}
impl std::fmt::Display for SubscriptionShippingUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BillingInfoCreate {
    pub gateway_token: Option<String>,
    ///*STRONGLY RECOMMENDED*
    pub cvv: Option<String>,
    ///The payment method type for a non-credit card based billing info. `bacs` and `becs` are the only accepted values.
    pub type_: Option<String>,
    pub fraud_session_id: Option<String>,
    pub first_name: Option<String>,
    pub card_type: Option<String>,
    ///Use for Adyen HPP billing info. This should only be used as part of a pending purchase request, when the billing info is nested inside an account object.
    pub external_hpp_type: Option<String>,
    ///Use for Online Banking billing info. This should only be used as part of a pending purchase request, when the billing info is nested inside an account object.
    pub online_banking_payment_type: Option<String>,
    ///Credit card number, spaces and dashes are accepted.
    pub number: Option<String>,
    pub address: Option<Address>,
    pub month: Option<String>,
    ///*STRONGLY RECOMMENDED* Customer's IP address when updating their billing information.
    pub ip_address: Option<String>,
    ///Tax identifier is required if adding a billing info that is a consumer card in Brazil or in Argentina. This would be the customer's CPF/CNPJ (Brazil) and CUIT (Argentina). CPF, CNPJ and CUIT are tax identifiers for all residents who pay taxes in Brazil and Argentina respectively.
    pub tax_identifier: Option<String>,
    pub amazon_billing_agreement_id: Option<String>,
    pub gateway_code: Option<String>,
    ///A token [generated by Recurly.js](https://recurly.com/developers/reference/recurly-js/#getting-a-token).
    pub token_id: Option<String>,
    ///This field and a value of `cpf`, `cnpj` or `cuit` are required if adding a billing info that is an elo or hipercard type in Brazil or in Argentina.
    pub tax_identifier_type: Option<String>,
    pub vat_number: Option<String>,
    pub last_name: Option<String>,
    pub company: Option<String>,
    ///A token generated by Recurly.js after completing a 3-D Secure device fingerprinting or authentication challenge.
    pub three_d_secure_action_result_token_id: Option<String>,
    ///The name associated with the bank account (ACH, SEPA, Bacs only)
    pub name_on_account: Option<String>,
    pub year: Option<String>,
    ///The bank account number. (ACH, Bacs only)
    pub account_number: Option<String>,
    ///The bank account type. (ACH only)
    pub account_type: Option<String>,
    ///The bank's rounting number. (ACH only)
    pub routing_number: Option<String>,
    ///The `primary_payment_method` field is used to designate the primary billing info on the account. The first billing info created on an account will always become primary. Adding additional billing infos provides the flexibility to mark another billing info as primary, or adding additional non-primary billing infos. This can be accomplished by passing the `primary_payment_method` with a value of `true`. When adding billing infos via the billing_info and /accounts endpoints, this value is not permitted, and will return an error if provided.
    pub primary_payment_method: Option<bool>,
    ///An optional type designation for the payment gateway transaction created by this request. Supports 'moto' value, which is the acronym for mail order and telephone transactions.
    pub transaction_type: Option<String>,
    pub paypal_billing_agreement_id: Option<String>,
    ///Bank identifier code for UK based banks. Required for Bacs based billing infos. (Bacs only)
    pub sort_code: Option<String>,
    ///The `backup_payment_method` field is used to designate a billing info as a backup on the account that will be tried if the initial billing info used for an invoice is declined. All payment methods, including the billing info marked `primary_payment_method` can be set as a backup. An account can have a maximum of 1 backup, if a user sets a different payment method as a backup, the existing backup will no longer be marked as such.
    pub backup_payment_method: Option<bool>,
    ///The International Bank Account Number, up to 34 alphanumeric characters comprising a country code; two check digits; and a number that includes the domestic bank account number, branch identifier, and potential routing information
    pub iban: Option<String>,
}
impl std::fmt::Display for BillingInfoCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountAcquisitionReadOnly {
    ///When the account acquisition data was created.
    pub created_at: Option<String>,
    pub object: Option<String>,
    ///When the account acquisition data was last changed.
    pub updated_at: Option<String>,
    pub id: Option<String>,
    pub account: Option<AccountMini>,
}
impl std::fmt::Display for AccountAcquisitionReadOnly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TierPricing {
    ///Allows up to 2 decimal places. Required unless `unit_amount_decimal` is provided.
    pub unit_amount: Option<f64>,
    /**Allows up to 9 decimal places. Only supported when `add_on_type` = `usage`.
If `unit_amount_decimal` is provided, `unit_amount` cannot be provided.
*/
    pub unit_amount_decimal: Option<String>,
    ///3-letter ISO 4217 currency code.
    pub currency: String,
}
impl std::fmt::Display for TierPricing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionShipping {
    pub object: Option<String>,
    pub method: Option<ShippingMethodMini>,
    pub address: Option<ShippingAddress>,
    pub amount: Option<f64>,
}
impl std::fmt::Display for SubscriptionShipping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LineItem {
    ///Used by Avalara, Vertex, and Recurly’s EU VAT tax feature. The tax code values are specific to each tax system. If you are using Recurly’s EU VAT feature you can use `unknown`, `physical`, or `digital`.
    pub tax_code: Option<String>,
    ///Will only have a value if the line item is a credit created from a previous credit, or if the credit was created from a charge refund.
    pub previous_line_item_id: Option<String>,
    pub id: Option<String>,
    ///For plan-related line items this will be the plan's code, for add-on related line items it will be the add-on's code. For item-related line items it will be the item's `external_sku`.
    pub product_code: Option<String>,
    ///3-letter ISO 4217 currency code.
    pub currency: Option<String>,
    ///This number will be multiplied by the unit amount to compute the subtotal before any discounts or taxes.
    pub quantity: Option<i64>,
    pub object: Option<String>,
    ///A floating-point alternative to Quantity. If this value is present, it will be used in place of Quantity for calculations, and Quantity will be the rounded integer value of this number. This field supports up to 9 decimal places. The Decimal Quantity feature must be enabled to utilize this field.
    pub quantity_decimal: Option<String>,
    ///The UUID is useful for matching data with the CSV exports and building URLs into Recurly's UI.
    pub uuid: Option<String>,
    ///Positive amount for a charge, negative amount for a credit.
    pub unit_amount_decimal: Option<String>,
    ///If the line item is a charge or credit for an add-on, this is its code.
    pub add_on_code: Option<String>,
    ///The invoice where the credit originated. Will only have a value if the line item is a credit created from a previous credit, or if the credit was created from a charge refund.
    pub original_line_item_invoice_id: Option<String>,
    ///The UUID of the account responsible for originating the line item.
    pub bill_for_account_id: Option<String>,
    ///The reason the credit was given when line item is `type=credit`.
    pub credit_reason_code: Option<String>,
    pub revenue_schedule_type: Option<String>,
    pub account: Option<AccountMini>,
    ///System-generated unique identifier for an item. Available when the Credit Invoices feature is enabled.
    pub item_id: Option<String>,
    ///If the line item is a charge or credit for an add-on this is its ID.
    pub add_on_id: Option<String>,
    ///Once the line item has been invoiced this will be the invoice's number. If VAT taxation and the Country Invoice Sequencing feature are enabled, invoices will have country-specific invoice numbers for invoices billed to EU countries (ex: FR1001). Non-EU invoices will continue to use the site-level invoice number sequence.
    pub invoice_number: Option<String>,
    ///`(quantity * unit_amount) - (discount + tax)`
    pub amount: Option<f64>,
    ///`quantity * unit_amount`
    pub subtotal: Option<f64>,
    ///`true` exempts tax on charges, `false` applies tax on charges. If not defined, then defaults to the Plan and Site settings. This attribute does not work for credits (negative line items). Credits are always applied post-tax. Pre-tax discounts should use the Coupons feature.
    pub tax_exempt: Option<bool>,
    ///When a line item has been prorated, this is the rate of the proration. Proration rates were made available for line items created after March 30, 2017. For line items created prior to that date, the proration rate will be `null`, even if the line item was prorated.
    pub proration_rate: Option<f64>,
    ///Determines whether or not tax is included in the unit amount. The Tax Inclusive Pricing feature (separate from the Mixed Tax Pricing feature) must be enabled to utilize this flag.
    pub tax_inclusive: Option<bool>,
    pub refund: Option<bool>,
    ///For refund charges, the quantity being refunded. For non-refund charges, the total quantity refunded (possibly over multiple refunds).
    pub refunded_quantity: Option<i64>,
    ///When the line item was created.
    pub created_at: Option<String>,
    ///Optional Stock Keeping Unit assigned to an item. Available when the Credit Invoices feature is enabled.
    pub external_sku: Option<String>,
    ///If this date is provided, it indicates the end of a time range.
    pub end_date: Option<String>,
    pub tax_info: Option<TaxInfo>,
    ///Internal accounting code to help you reconcile your revenue to the correct ledger. Line items created as part of a subscription invoice will use the plan or add-on's accounting code, otherwise the value will only be present if you define an accounting code when creating the line item.
    pub accounting_code: Option<String>,
    ///If the line item is a charge or credit for a plan or add-on, this is the plan's code.
    pub plan_code: Option<String>,
    /**Category to describe the role of a line item on a legacy invoice:
- "charges" refers to charges being billed for on this invoice.
- "credits" refers to refund or proration credits. This portion of the invoice can be considered a credit memo.
- "applied_credits" refers to previous credits applied to this invoice. See their original_line_item_id to determine where the credit first originated.
- "carryforwards" can be ignored. They exist to consume any remaining credit balance. A new credit with the same amount will be created and placed back on the account.
*/
    pub legacy_category: Option<String>,
    ///Positive amount for a charge, negative amount for a credit.
    pub unit_amount: Option<f64>,
    ///Charges are positive line items that debit the account. Credits are negative line items that credit the account.
    pub type_: Option<String>,
    ///Once the line item has been invoiced this will be the invoice's ID.
    pub invoice_id: Option<String>,
    ///If the line item is a charge or credit for a subscription, this is its ID.
    pub subscription_id: Option<String>,
    ///If the line item is a charge or credit for a plan or add-on, this is the plan's ID.
    pub plan_id: Option<String>,
    ///A credit created from an original charge will have the value of the charge's origin.
    pub origin: Option<String>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the line item is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types.
    pub avalara_transaction_type: Option<i64>,
    ///A floating-point alternative to Refunded Quantity. For refund charges, the quantity being refunded. For non-refund charges, the total quantity refunded (possibly over multiple refunds). The Decimal Quantity feature must be enabled to utilize this field.
    pub refunded_quantity_decimal: Option<String>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the line item is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types.
    pub avalara_service_type: Option<i64>,
    ///The amount of credit from this line item that was applied to the invoice.
    pub credit_applied: Option<f64>,
    ///Pending line items are charges or credits on an account that have not been applied to an invoice yet. Invoiced line items will always have an `invoice_id` value.
    pub state: Option<String>,
    pub shipping_address: Option<ShippingAddress>,
    ///The discount applied to the line item.
    pub discount: Option<f64>,
    ///`true` if the line item is taxable, `false` if it is not.
    pub taxable: Option<bool>,
    ///If an end date is present, this is value indicates the beginning of a billing time range. If no end date is present it indicates billing for a specific date.
    pub start_date: Option<String>,
    ///When the line item was last changed.
    pub updated_at: Option<String>,
    ///Unique code to identify an item. Available when the Credit Invoices feature is enabled.
    pub item_code: Option<String>,
    ///Description that appears on the invoice. For subscription related items this will be filled in automatically.
    pub description: Option<String>,
    ///The tax amount for the line item.
    pub tax: Option<f64>,
}
impl std::fmt::Display for LineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubscriptionAddOnPercentageTier {
    /**The percentage taken of the monetary amount of usage tracked.
This can be up to 4 decimal places represented as a string.
*/
    pub usage_percentage: Option<String>,
    ///Ending amount for the tier. Allows up to 2 decimal places. Must be left empty if it is the final tier.
    pub ending_amount: Option<f64>,
}
impl std::fmt::Display for SubscriptionAddOnPercentageTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    ///Unique ID to identify a dunning campaign. Used to specify if a non-default dunning campaign should be assigned to this plan. For sites without multiple dunning campaigns enabled, the default dunning campaign will always be used.
    pub dunning_campaign_id: Option<String>,
    ///The custom fields will only be altered when they are included in a request. Sending an empty array will not remove any existing values. To remove a field send the name with a null or empty value.
    pub custom_fields: Option<CustomFields>,
    ///Accounting code for invoice line items for the plan's setup fee. If no value is provided, it defaults to plan's accounting code.
    pub setup_fee_accounting_code: Option<String>,
    ///Optional description, not displayed.
    pub description: Option<String>,
    pub created_at: Option<String>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the plan is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types.
    pub avalara_transaction_type: Option<i64>,
    ///The current state of the plan.
    pub state: Option<String>,
    pub ramp_intervals: Option<Vec<PlanRampInterval>>,
    ///This name describes your plan and will appear on the Hosted Payment Page and the subscriber's invoice.
    pub name: String,
    /**A fixed pricing model has the same price for each billing period.
A ramp pricing model defines a set of Ramp Intervals, where a subscription changes price on
a specified cadence of billing periods. The price change could be an increase or decrease.
*/
    pub pricing_model: Option<String>,
    ///Units for the plan's trial period.
    pub trial_unit: Option<String>,
    ///Used by Avalara, Vertex, and Recurly’s EU VAT tax feature. The tax code values are specific to each tax system. If you are using Recurly’s EU VAT feature you can use `unknown`, `physical`, or `digital`.
    pub tax_code: Option<String>,
    ///Unique code to identify the plan. This is used in Hosted Payment Page URLs and in the invoice exports.
    pub code: String,
    pub revenue_schedule_type: Option<String>,
    ///Unit for the plan's billing interval.
    pub interval_unit: Option<String>,
    ///Subscriptions will automatically inherit this value once they are active. If `auto_renew` is `true`, then a subscription will automatically renew its term at renewal. If `auto_renew` is `false`, then a subscription will expire at the end of its term. `auto_renew` can be overridden on the subscription record itself.
    pub auto_renew: Option<bool>,
    pub hosted_pages: Option<PlanHostedPages>,
    ///Length of plan's trial period in `trial_units`. `0` means `no trial`.
    pub trial_length: Option<i64>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the plan is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types.
    pub avalara_service_type: Option<i64>,
    ///Allow free trial subscriptions to be created without billing info. Should not be used if billing info is needed for initial invoice due to existing uninvoiced charges or setup fee.
    pub trial_requires_billing_info: Option<bool>,
    pub setup_fee_revenue_schedule_type: Option<String>,
    ///Length of the plan's billing interval in `interval_unit`.
    pub interval_length: Option<i64>,
    ///`true` exempts tax on the plan, `false` applies tax on the plan.
    pub tax_exempt: Option<bool>,
    pub id: Option<String>,
    pub updated_at: Option<String>,
    pub object: Option<String>,
    ///Accounting code for invoice line items for the plan. If no value is provided, it defaults to plan's code.
    pub accounting_code: Option<String>,
    ///Automatically terminate subscriptions after a defined number of billing cycles. Number of billing cycles before the plan automatically stops renewing, defaults to `null` for continuous, automatic renewal.
    pub total_billing_cycles: Option<i64>,
    pub currencies: Option<Vec<PlanPricing>>,
    /**Used to determine whether items can be assigned as add-ons to individual subscriptions.
If `true`, items can be assigned as add-ons to individual subscription add-ons.
If `false`, only plan add-ons can be used.
*/
    pub allow_any_item_on_subscriptions: Option<bool>,
    pub deleted_at: Option<String>,
}
impl std::fmt::Display for Plan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExportDates {
    pub object: Option<String>,
    pub dates: Option<Vec<String>>,
}
impl std::fmt::Display for ExportDates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ItemMini {
    pub id: Option<String>,
    ///Unique code to identify the item.
    pub code: Option<String>,
    ///Optional, description.
    pub description: Option<String>,
    pub object: Option<String>,
    ///This name describes your item and will appear on the invoice when it's purchased on a one time basis.
    pub name: Option<String>,
    ///The current state of the item.
    pub state: Option<String>,
}
impl std::fmt::Display for ItemMini {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AddOnUpdate {
    ///Name of a measured unit to be associated with the add-on. Either `measured_unit_id` or `measured_unit_name` are required when `add_on_type` is `usage`. If `measured_unit_id` and `measured_unit_name` are both present, `measured_unit_id` will be used.
    pub measured_unit_name: Option<String>,
    ///Optional field used by Avalara, Vertex, and Recurly's EU VAT tax feature to determine taxation rules. If you have your own AvaTax or Vertex account configured, use their tax codes to assign specific tax rules. If you are using Recurly's EU VAT feature, you can use values of `unknown`, `physical`, or `digital`. If an `Item` is associated to the `AddOn` then `tax code` must be absent.
    pub tax_code: Option<String>,
    /**If the tier_type is `flat`, then `tiers` must be absent. The `tiers` object
must include one to many tiers with `ending_quantity` and `unit_amount` for
the desired `currencies`. There must be one tier without an `ending_quantity` value
which represents the final tier.
*/
    pub tiers: Option<Vec<Tier>>,
    /**If the add-on's `tier_type` is `tiered`, `volume`, or `stairstep`,
then currencies must be absent. Must also be absent if `add_on_type` is
`usage` and `usage_type` is `percentage`.
*/
    pub currencies: Option<Vec<AddOnPricing>>,
    /**`percentage_tiers` is an array of objects, which must have the set of tiers
per currency and the currency code. The tier_type must be `volume` or `tiered`,
if not, it must be absent. There must be one tier without an `ending_amount` value
which represents the final tier.
*/
    pub percentage_tiers: Option<Vec<PercentageTiersByCurrency>>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the add-on is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types. If an `Item` is associated to the `AddOn`, then the `avalara_service_type` must be absent.
    pub avalara_service_type: Option<i64>,
    ///Accounting code for invoice line items for this add-on. If no value is provided, it defaults to add-on's code. If an `Item` is associated to the `AddOn` then `accounting code` must be absent.
    pub accounting_code: Option<String>,
    ///The percentage taken of the monetary amount of usage tracked. This can be up to 4 decimal places. A value between 0.0 and 100.0. Required if `add_on_type` is usage, `tier_type` is `flat` and `usage_type` is percentage. Must be omitted otherwise.
    pub usage_percentage: Option<f64>,
    ///Describes your add-on and will appear in subscribers' invoices. If an `Item` is associated to the `AddOn` then `name` must be absent.
    pub name: Option<String>,
    ///The type of calculation to be employed for an add-on.  Cumulative billing will sum all usage records created in the current billing cycle.  Last-in-period billing will apply only the most recent usage record in the billing period.  If no value is specified, cumulative billing will be used.
    pub usage_calculation_type: Option<String>,
    ///System-generated unique identifier for a measured unit to be associated with the add-on. Either `measured_unit_id` or `measured_unit_name` are required when `add_on_type` is `usage`. If `measured_unit_id` and `measured_unit_name` are both present, `measured_unit_id` will be used.
    pub measured_unit_id: Option<String>,
    pub id: Option<String>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the add-on is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types. If an `Item` is associated to the `AddOn`, then the `avalara_transaction_type` must be absent.
    pub avalara_transaction_type: Option<i64>,
    ///Determines if the quantity field is displayed on the hosted pages for the add-on.
    pub display_quantity: Option<bool>,
    ///The unique identifier for the add-on within its plan. If an `Item` is associated to the `AddOn` then `code` must be absent.
    pub code: Option<String>,
    ///Default quantity for the hosted pages.
    pub default_quantity: Option<i64>,
    ///Whether the add-on is optional for the customer to include in their purchase on the hosted payment page. If false, the add-on will be included when a subscription is created through the Recurly UI. However, the add-on will not be included when a subscription is created through the API.
    pub optional: Option<bool>,
    ///When this add-on is invoiced, the line item will use this revenue schedule. If `item_code`/`item_id` is part of the request then `revenue_schedule_type` must be absent in the request as the value will be set from the item.
    pub revenue_schedule_type: Option<String>,
}
impl std::fmt::Display for AddOnUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DunningCampaignsBulkUpdate(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShippingMethodMini {
    pub object: Option<String>,
    ///The internal name used identify the shipping method.
    pub code: Option<String>,
    pub id: Option<String>,
    ///The name of the shipping method displayed to customers.
    pub name: Option<String>,
}
impl std::fmt::Display for ShippingMethodMini {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionShippingCreate {
    ///The code of the shipping method used to deliver the subscription. If `method_id` and `method_code` are both present, `method_id` will be used.
    pub method_code: Option<String>,
    ///Assign a shipping address from the account's existing shipping addresses. If `address_id` and `address` are both present, `address` will be used.
    pub address_id: Option<String>,
    pub amount: Option<f64>,
    ///The id of the shipping method used to deliver the subscription. If `method_id` and `method_code` are both present, `method_id` will be used.
    pub method_id: Option<String>,
    pub address: Option<ShippingAddressCreate>,
}
impl std::fmt::Display for SubscriptionShippingCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Site {
    pub object: Option<String>,
    pub settings: Option<Settings>,
    pub deleted_at: Option<String>,
    pub subdomain: Option<String>,
    pub address: Option<Address>,
    ///A list of features enabled for the site.
    pub features: Option<Vec<String>>,
    pub created_at: Option<String>,
    pub id: Option<String>,
    pub updated_at: Option<String>,
    ///This value is used to configure RecurlyJS to submit tokenized billing information.
    pub public_api_key: Option<String>,
    pub mode: Option<String>,
}
impl std::fmt::Display for Site {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CouponRedemptionCreate {
    pub coupon_id: String,
    ///3-letter ISO 4217 currency code.
    pub currency: Option<String>,
    pub subscription_id: Option<String>,
}
impl std::fmt::Display for CouponRedemptionCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PercentageTiersByCurrency {
    ///3-letter ISO 4217 currency code.
    pub currency: Option<String>,
    pub tiers: Option<Vec<PercentageTier>>,
}
impl std::fmt::Display for PercentageTiersByCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlanList {
    ///Will always be List.
    pub object: Option<String>,
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
    pub data: Option<Vec<Plan>>,
}
impl std::fmt::Display for PlanList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CouponCreate {
    ///The coupon is valid for one-time, non-plan charges if true.
    pub applies_to_non_plan_charges: bool,
    ///If `duration` is "temporal" than `temporal_amount` is an integer which is multiplied by `temporal_unit` to define the duration that the coupon will be applied to invoices for.
    pub temporal_amount: i64,
    pub coupon_update: CouponUpdate,
    /**List of item codes to which this coupon applies. Sending
`item_codes` is only permitted when `applies_to_all_items` is set to false.
The following values are not permitted when `item_codes` is included:
`free_trial_amount` and `free_trial_unit`.
*/
    pub item_codes: Vec<String>,
    /**This field does not apply when the discount_type is `free_trial`.
- "single_use" coupons applies to the first invoice only.
- "temporal" coupons will apply to invoices for the duration determined by the `temporal_unit` and `temporal_amount` attributes.
- "forever" coupons will apply to invoices forever.
*/
    pub duration: String,
    ///Whether the discount is for all eligible charges on the account, or only a specific subscription.
    pub redemption_resource: String,
    ///The coupon is valid for all plans if true. If false then `plans` will list the applicable plans.
    pub applies_to_all_plans: bool,
    ///Description of the unit of time the coupon is for. Used with `free_trial_amount` to determine the duration of time the coupon is for.  Required if `discount_type` is `free_trial`.
    pub free_trial_unit: String,
    /**To apply coupon to Items in your Catalog, include a list
of `item_codes` in the request that the coupon will apply to. Or set value
to true to apply to all Items in your Catalog. The following values
are not permitted when `applies_to_all_items` is included: `free_trial_amount`
and `free_trial_unit`.
*/
    pub applies_to_all_items: bool,
    /**On a bulk coupon, the template from which unique coupon codes are generated.
- You must start the template with your coupon_code wrapped in single quotes.
- Outside of single quotes, use a 9 for a character that you want to be a random number.
- Outside of single quotes, use an "x" for a character that you want to be a random letter.
- Outside of single quotes, use an * for a character that you want to be a random number or letter.
- Use single quotes ' ' for characters that you want to remain static. These strings can be alphanumeric and may contain a - _ or +.
For example: "'abc-'****'-def'"
*/
    pub unique_code_template: String,
    ///The percent of the price discounted by the coupon.  Required if `discount_type` is `percent`.
    pub discount_percent: i64,
    ///The code the customer enters to redeem the coupon.
    pub code: String,
    ///Whether the coupon is "single_code" or "bulk". Bulk coupons will require a `unique_code_template` and will generate unique codes through the `/generate` endpoint.
    pub coupon_type: String,
    ///Sets the duration of time the `free_trial_unit` is for. Required if `discount_type` is `free_trial`.
    pub free_trial_amount: i64,
    ///Fixed discount currencies by currency. Required if the coupon type is `fixed`. This parameter should contain the coupon discount values
    pub currencies: Vec<CouponPricing>,
    ///If `duration` is "temporal" than `temporal_unit` is multiplied by `temporal_amount` to define the duration that the coupon will be applied to invoices for.
    pub temporal_unit: String,
    /**List of plan codes to which this coupon applies. Required
if `applies_to_all_plans` is false. Overrides `applies_to_all_plans`
when `applies_to_all_plans` is true.
*/
    pub plan_codes: Vec<String>,
    ///The type of discount provided by the coupon (how the amount discounted is calculated)
    pub discount_type: String,
}
impl std::fmt::Display for CouponCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceUpdate {
    ///This identifies the PO number associated with the invoice. Not editable for credit invoices.
    pub po_number: Option<String>,
    ///Integer representing the number of days after an invoice's creation that the invoice will become past due. Changing Net terms changes due_on, and the invoice could move between past due and pending.
    pub net_terms: Option<i64>,
    ///VAT Reverse Charge Notes are editable only if there was a VAT reverse charge applied to the invoice.
    pub vat_reverse_charge_notes: Option<String>,
    ///Terms and conditions are an optional note field. Not editable for credit invoices.
    pub terms_and_conditions: Option<String>,
    pub address: Option<InvoiceAddress>,
    ///Customer notes are an optional note field.
    pub customer_notes: Option<String>,
}
impl std::fmt::Display for InvoiceUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubscriptionChangeBillingInfo {
    ///A token generated by Recurly.js after completing a 3-D Secure device fingerprinting or authentication challenge.
    pub three_d_secure_action_result_token_id: Option<String>,
}
impl std::fmt::Display for SubscriptionChangeBillingInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubscriptionAddOnUpdate {
    /**When an id is provided, the existing subscription add-on attributes will
persist unless overridden in the request.
*/
    pub id: Option<String>,
    pub quantity: Option<i64>,
    /**If the plan add-on's `tier_type` is `flat`, then `tiers` must be absent. The `tiers` object
must include one to many tiers with `ending_quantity` and `unit_amount`.
There must be one tier without an `ending_quantity` value which represents the final tier.
*/
    pub tiers: Option<Vec<SubscriptionAddOnTier>>,
    ///The percentage taken of the monetary amount of usage tracked. This can be up to 4 decimal places. A value between 0.0 and 100.0. Required if add_on_type is usage and usage_type is percentage.
    pub usage_percentage: Option<f64>,
    pub revenue_schedule_type: Option<String>,
    /**Used to determine where the associated add-on data is pulled from. If this value is set to
`plan_add_on` or left blank, then add-on data will be pulled from the plan's add-ons. If the associated
`plan` has `allow_any_item_on_subscriptions` set to `true` and this field is set to `item`, then
the associated add-on data will be pulled from the site's item catalog.
*/
    pub add_on_source: Option<String>,
    /**If a code is provided without an id, the subscription add-on attributes
will be set to the current value for those attributes on the plan add-on
unless provided in the request. If `add_on_source` is set to `plan_add_on`
or left blank, then plan's add-on `code` should be used. If `add_on_source`
is set to `item`, then the `code` from the associated item should be used.
*/
    pub code: Option<String>,
    /**Allows up to 9 decimal places. Optionally, override the add-on's default unit amount.
If the plan add-on's `tier_type` is `tiered`, `volume`, or `stairstep`, then `unit_amount_decimal` cannot be provided.
Only supported when the plan add-on's `add_on_type` = `usage`.
If `unit_amount_decimal` is provided, `unit_amount` cannot be provided.
*/
    pub unit_amount_decimal: Option<String>,
    /**If percentage tiers are provided in the request, all existing percentage tiers on the Subscription Add-on will be
removed and replaced by the percentage tiers in the request. Use only if add_on.tier_type is tiered or volume and
add_on.usage_type is percentage.
There must be one tier without an `ending_amount` value which represents the final tier.
*/
    pub percentage_tiers: Option<Vec<SubscriptionAddOnPercentageTier>>,
    /**Allows up to 2 decimal places. Optionally, override the add-on's default unit amount.
If the plan add-on's `tier_type` is `tiered`, `volume`, or `stairstep`, then `unit_amount` cannot be provided.
*/
    pub unit_amount: Option<f64>,
}
impl std::fmt::Display for SubscriptionAddOnUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InvoiceList {
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    pub data: Option<Vec<Invoice>>,
    ///Will always be List.
    pub object: Option<String>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
}
impl std::fmt::Display for InvoiceList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExternalProductList {
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    ///Will always be List.
    pub object: Option<String>,
    pub data: Option<Vec<ExternalProduct>>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
}
impl std::fmt::Display for ExternalProductList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomFields(pub Vec<CustomField>);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountRequired {
    pub preferred_locale: String,
    pub first_name: String,
    pub company: String,
    pub code: String,
    pub tax_exempt: bool,
    pub email: String,
    pub exemption_certificate: String,
    pub parent_account_id: String,
    pub transaction_type: String,
    pub acquisition: AccountAcquisitionUpdate,
    pub dunning_campaign_id: String,
    pub address: Address,
    pub vat_number: String,
    pub bill_to: String,
    pub billing_info: BillingInfoCreate,
    pub parent_account_code: String,
    pub invoice_template_id: String,
    pub cc_emails: String,
    pub last_name: String,
    pub shipping_addresses: Vec<ShippingAddressCreate>,
    pub username: String,
    pub custom_fields: CustomFields,
}
impl std::fmt::Display for CreateAccountRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CouponRedemptionMini {
    pub created_at: Option<String>,
    pub state: Option<String>,
    ///Will always be `coupon`.
    pub object: Option<String>,
    pub id: Option<String>,
    pub coupon: Option<CouponMini>,
    ///The amount that was discounted upon the application of the coupon, formatted with the currency.
    pub discounted: Option<f64>,
}
impl std::fmt::Display for CouponRedemptionMini {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShippingAddress {
    ///Country, 2-letter ISO 3166-1 alpha-2 code.
    pub country: Option<String>,
    pub vat_number: Option<String>,
    pub first_name: Option<String>,
    pub email: Option<String>,
    pub id: Option<String>,
    ///Zip or postal code.
    pub postal_code: Option<String>,
    pub updated_at: Option<String>,
    pub company: Option<String>,
    pub nickname: Option<String>,
    pub account_id: Option<String>,
    pub street1: Option<String>,
    pub street2: Option<String>,
    pub created_at: Option<String>,
    pub object: Option<String>,
    ///State or province.
    pub region: Option<String>,
    pub phone: Option<String>,
    pub last_name: Option<String>,
    pub city: Option<String>,
}
impl std::fmt::Display for ShippingAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubscriptionPause {
    ///Number of billing cycles to pause the subscriptions. A value of 0 will cancel any pending pauses on the subscription.
    pub remaining_pause_cycles: i64,
}
impl std::fmt::Display for SubscriptionPause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateCouponRequired {
    pub plan_codes: Vec<String>,
    pub unique_code_template: String,
    pub free_trial_unit: String,
    pub currencies: Vec<CouponPricing>,
    pub redeem_by_date: String,
    pub applies_to_all_plans: bool,
    pub applies_to_all_items: bool,
    pub applies_to_non_plan_charges: bool,
    pub name: String,
    pub discount_type: String,
    pub discount_percent: i64,
    pub max_redemptions_per_account: i64,
    pub free_trial_amount: i64,
    pub code: String,
    pub coupon_type: String,
    pub max_redemptions: i64,
    pub item_codes: Vec<String>,
    pub temporal_unit: String,
    pub hosted_description: String,
    pub invoice_description: String,
    pub duration: String,
    pub temporal_amount: i64,
    pub redemption_resource: String,
}
impl std::fmt::Display for CreateCouponRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AccountList {
    ///Path to subsequent page of results.
    pub next: Option<String>,
    pub data: Option<Vec<Account>>,
    ///Will always be List.
    pub object: Option<String>,
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
}
impl std::fmt::Display for AccountList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubscriptionAddOnTier {
    ///Ending quantity for the tier.  This represents a unit amount for unit-priced add ons. Must be left empty if it is the final tier.
    pub ending_quantity: Option<i64>,
    ///(deprecated) -- Use the percentage_tiers object instead.
    pub usage_percentage: Option<String>,
    ///Allows up to 2 decimal places. Optionally, override the tiers' default unit amount. If add-on's `add_on_type` is `usage` and `usage_type` is `percentage`, cannot be provided.
    pub unit_amount: Option<f64>,
    /**Allows up to 9 decimal places.  Optionally, override tiers' default unit amount.
If `unit_amount_decimal` is provided, `unit_amount` cannot be provided.
If add-on's `add_on_type` is `usage` and `usage_type` is `percentage`, cannot be provided.
*/
    pub unit_amount_decimal: Option<String>,
}
impl std::fmt::Display for SubscriptionAddOnTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethod(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MeasuredUnitCreate {
    ///Unique internal name of the measured unit on your site.
    pub name: String,
    ///Display name for the measured unit.
    pub display_name: String,
    ///Optional internal description.
    pub description: Option<String>,
}
impl std::fmt::Display for MeasuredUnitCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExternalResourceMini {
    ///System-generated unique identifier for an external resource ID, e.g. `e28zov4fw0v2`.
    pub id: Option<String>,
    pub object: Option<String>,
    ///Identifier or URL reference where the resource is canonically available in the external platform.
    pub external_object_reference: Option<String>,
}
impl std::fmt::Display for ExternalResourceMini {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MeasuredUnit {
    ///The current state of the measured unit.
    pub state: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
    pub id: Option<String>,
    ///Unique internal name of the measured unit on your site.
    pub name: Option<String>,
    ///Optional internal description.
    pub description: Option<String>,
    ///Display name for the measured unit. Can only contain spaces, underscores and must be alphanumeric.
    pub display_name: Option<String>,
    pub created_at: Option<String>,
    pub object: Option<String>,
}
impl std::fmt::Display for MeasuredUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AccountAcquisitionList {
    ///Path to subsequent page of results.
    pub next: Option<String>,
    ///Will always be List.
    pub object: Option<String>,
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    pub data: Option<Vec<AccountAcquisition>>,
}
impl std::fmt::Display for AccountAcquisitionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InvoiceCreate {
    ///This will default to the Customer Notes text specified on the Invoice Settings for charge invoices. Specify custom notes to add or override Customer Notes on charge invoices.
    pub charge_customer_notes: Option<String>,
    ///This will default to the Terms and Conditions text specified on the Invoice Settings page in your Recurly admin. Specify custom notes to add or override Terms and Conditions.
    pub terms_and_conditions: Option<String>,
    ///For manual invoicing, this identifies the PO number associated with the subscription.
    pub po_number: Option<String>,
    ///VAT Reverse Charge Notes only appear if you have EU VAT enabled or are using your own Avalara AvaTax account and the customer is in the EU, has a VAT number, and is in a different country than your own. This will default to the VAT Reverse Charge Notes text specified on the Tax Settings page in your Recurly admin, unless custom notes were created with the original subscription.
    pub vat_reverse_charge_notes: Option<String>,
    ///3-letter ISO 4217 currency code.
    pub currency: String,
    ///This will default to the Customer Notes text specified on the Invoice Settings for credit invoices. Specify customer notes to add or override Customer Notes on credit invoices.
    pub credit_customer_notes: Option<String>,
    ///Integer representing the number of days after an invoice's creation that the invoice will become past due. If an invoice's net terms are set to '0', it is due 'On Receipt' and will become past due 24 hours after it’s created. If an invoice is due net 30, it will become past due at 31 days exactly.
    pub net_terms: Option<i64>,
    ///An automatic invoice means a corresponding transaction is run using the account's billing information at the same time the invoice is created. Manual invoices are created without a corresponding transaction. The merchant must enter a manual payment transaction or have the customer pay the invoice with an automatic method, like credit card, PayPal, Amazon, or ACH bank payment.
    pub collection_method: Option<String>,
}
impl std::fmt::Display for InvoiceCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShippingAddressUpdate {
    pub city: Option<String>,
    pub last_name: Option<String>,
    pub nickname: Option<String>,
    pub street2: Option<String>,
    pub id: Option<String>,
    ///Country, 2-letter ISO 3166-1 alpha-2 code.
    pub country: Option<String>,
    pub street1: Option<String>,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub company: Option<String>,
    ///State or province.
    pub region: Option<String>,
    ///Zip or postal code.
    pub postal_code: Option<String>,
    pub email: Option<String>,
    pub vat_number: Option<String>,
}
impl std::fmt::Display for ShippingAddressUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Usage {
    ///The tiers and prices of the subscription based on the usage_timestamp. If tier_type = flat, tiers = []
    pub tiers: Option<Vec<SubscriptionAddOnTier>>,
    ///When the usage was recorded in your system.
    pub recording_timestamp: Option<String>,
    ///When the usage record was billed on an invoice.
    pub billed_at: Option<String>,
    ///The percentage taken of the monetary amount of usage tracked. This can be up to 4 decimal places. A value between 0.0 and 100.0.
    pub usage_percentage: Option<f64>,
    ///When the usage record was billed on an invoice.
    pub updated_at: Option<String>,
    pub object: Option<String>,
    ///The ID of the measured unit associated with the add-on the usage record is for.
    pub measured_unit_id: Option<String>,
    ///When the usage record was created in Recurly.
    pub created_at: Option<String>,
    ///Type of usage, returns usage type if `add_on_type` is `usage`.
    pub usage_type: Option<String>,
    pub unit_amount: Option<f64>,
    ///The amount of usage. Can be positive, negative, or 0. If the Decimal Quantity feature is enabled, this value will be rounded to nine decimal places.  Otherwise, all digits after the decimal will be stripped. If the usage-based add-on is billed with a percentage, your usage should be a monetary amount formatted in cents (e.g., $5.00 is "500").
    pub amount: Option<f64>,
    /**The pricing model for the add-on.  For more information,
[click here](https://docs.recurly.com/docs/billing-models#section-quantity-based). See our
[Guide](https://recurly.com/developers/guides/item-addon-guide.html) for an overview of how
to configure quantity-based pricing models.
*/
    pub tier_type: Option<String>,
    ///The percentage tiers of the subscription based on the usage_timestamp. If tier_type = flat, percentage_tiers = []
    pub percentage_tiers: Option<Vec<SubscriptionAddOnPercentageTier>>,
    pub id: Option<String>,
    ///Unit price that can optionally support a sub-cent value.
    pub unit_amount_decimal: Option<String>,
    ///When the usage actually happened. This will define the line item dates this usage is billed under and is important for revenue recognition.
    pub usage_timestamp: Option<String>,
    ///Custom field for recording the id in your own system associated with the usage, so you can provide auditable usage displays to your customers using a GET on this endpoint.
    pub merchant_tag: Option<String>,
}
impl std::fmt::Display for Usage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BillingInfoVerify {
    ///An identifier for a specific payment gateway.
    pub gateway_code: Option<String>,
}
impl std::fmt::Display for BillingInfoVerify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InvoiceCollect {
    ///A token generated by Recurly.js after completing a 3-D Secure device fingerprinting or authentication challenge.
    pub three_d_secure_action_result_token_id: Option<String>,
    ///An optional type designation for the payment gateway transaction created by this request. Supports 'moto' value, which is the acronym for mail order and telephone transactions.
    pub transaction_type: Option<String>,
    ///The `billing_info_id` is the value that represents a specific billing info for an end customer. When `billing_info_id` is used to assign billing info to the subscription, all future billing events for the subscription will bill to the specified billing info. `billing_info_id` can ONLY be used for sites utilizing the Wallet feature.
    pub billing_info_id: Option<String>,
}
impl std::fmt::Display for InvoiceCollect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Pricing {
    ///This field is deprecated. Please do not use it.
    pub tax_inclusive: Option<bool>,
    ///3-letter ISO 4217 currency code.
    pub currency: String,
    pub unit_amount: f64,
}
impl std::fmt::Display for Pricing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UsageCreate {
    ///Custom field for recording the id in your own system associated with the usage, so you can provide auditable usage displays to your customers using a GET on this endpoint.
    pub merchant_tag: Option<String>,
    ///When the usage actually happened. This will define the line item dates this usage is billed under and is important for revenue recognition.
    pub usage_timestamp: Option<String>,
    ///The amount of usage. Can be positive, negative, or 0. If the Decimal Quantity feature is enabled, this value will be rounded to nine decimal places.  Otherwise, all digits after the decimal will be stripped. If the usage-based add-on is billed with a percentage, your usage should be a monetary amount formatted in cents (e.g., $5.00 is "500").
    pub amount: Option<f64>,
    ///When the usage was recorded in your system.
    pub recording_timestamp: Option<String>,
}
impl std::fmt::Display for UsageCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExternalTransaction {
    ///Payment method used for external transaction.
    pub payment_method: Option<String>,
    ///Used as the transaction's description.
    pub description: Option<String>,
    ///Datetime that the external payment was collected. Defaults to current datetime.
    pub collected_at: Option<String>,
    ///The total amount of the transcaction. Cannot excceed the invoice total.
    pub amount: Option<f64>,
}
impl std::fmt::Display for ExternalTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShippingMethodList {
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    ///Will always be List.
    pub object: Option<String>,
    pub data: Option<Vec<ShippingMethod>>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
}
impl std::fmt::Display for ShippingMethodList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AccountNoteList {
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    pub data: Option<Vec<AccountNote>>,
    ///Will always be List.
    pub object: Option<String>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
}
impl std::fmt::Display for AccountNoteList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlanHostedPages {
    ///If `true`, the customer will be sent directly to your `success_url` after a successful signup, bypassing Recurly's hosted confirmation page.
    pub bypass_confirmation: Option<bool>,
    ///URL to redirect to after signup on the hosted payment pages.
    pub success_url: Option<String>,
    ///URL to redirect to on canceled signup on the hosted payment pages.
    pub cancel_url: Option<String>,
    ///Determines if the quantity field is displayed on the hosted pages for the plan.
    pub display_quantity: Option<bool>,
}
impl std::fmt::Display for PlanHostedPages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubscriptionAddOnCreate {
    ///The percentage taken of the monetary amount of usage tracked. This can be up to 4 decimal places. A value between 0.0 and 100.0. Required if `add_on_type` is usage and `usage_type` is percentage. Must be omitted otherwise. `usage_percentage` does not support tiers. See our [Guide](https://recurly.com/developers/guides/usage-based-billing-guide.html) for an overview of how to configure usage add-ons.
    pub usage_percentage: Option<f64>,
    pub quantity: Option<i64>,
    /**If percentage tiers are provided in the request, all existing percentage tiers on the Subscription Add-on will be
removed and replaced by the percentage tiers in the request. There must be one tier without ending_amount value which represents the final tier.
Use only if add_on.tier_type is tiered or volume and add_on.usage_type is percentage.
*/
    pub percentage_tiers: Option<Vec<SubscriptionAddOnPercentageTier>>,
    /**Used to determine where the associated add-on data is pulled from. If this value is set to
`plan_add_on` or left blank, then add-on data will be pulled from the plan's add-ons. If the associated
`plan` has `allow_any_item_on_subscriptions` set to `true` and this field is set to `item`, then
the associated add-on data will be pulled from the site's item catalog.
*/
    pub add_on_source: Option<String>,
    /**If `add_on_source` is set to `plan_add_on` or left blank, then plan's add-on `code` should be used.
If `add_on_source` is set to `item`, then the `code` from the associated item should be used.
*/
    pub code: String,
    /**Allows up to 9 decimal places.  Optionally, override the add-on's default unit amount.
If the plan add-on's `tier_type` is `tiered`, `volume`, or `stairstep`, then `unit_amount_decimal` cannot be provided.
Only supported when the plan add-on's `add_on_type` = `usage`.
If `unit_amount_decimal` is provided, `unit_amount` cannot be provided.
*/
    pub unit_amount_decimal: Option<String>,
    /**If the plan add-on's `tier_type` is `flat`, then `tiers` must be absent. The `tiers` object
must include one to many tiers with `ending_quantity` and `unit_amount`.
There must be one tier without an `ending_quantity` value which represents the final tier.
See our [Guide](https://recurly.com/developers/guides/item-addon-guide.html)
for an overview of how to configure quantity-based pricing models.
*/
    pub tiers: Option<Vec<SubscriptionAddOnTier>>,
    /**Allows up to 2 decimal places. Optionally, override the add-on's default unit amount.
If the plan add-on's `tier_type` is `tiered`, `volume`, or `stairstep`, then `unit_amount` cannot be provided.
*/
    pub unit_amount: Option<f64>,
    pub revenue_schedule_type: Option<String>,
}
impl std::fmt::Display for SubscriptionAddOnCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemUpdate {
    pub currencies: Option<Vec<Pricing>>,
    ///Optional, description.
    pub description: Option<String>,
    pub revenue_schedule_type: Option<String>,
    ///Unique code to identify the item.
    pub code: Option<String>,
    ///This name describes your item and will appear on the invoice when it's purchased on a one time basis.
    pub name: Option<String>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the item is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types.
    pub avalara_service_type: Option<i64>,
    ///Used by Avalara, Vertex, and Recurly’s EU VAT tax feature. The tax code values are specific to each tax system. If you are using Recurly’s EU VAT feature you can use `unknown`, `physical`, or `digital`.
    pub tax_code: Option<String>,
    ///Optional, stock keeping unit to link the item to other inventory systems.
    pub external_sku: Option<String>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the item is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types.
    pub avalara_transaction_type: Option<i64>,
    ///`true` exempts tax on the item, `false` applies tax on the item.
    pub tax_exempt: Option<bool>,
    ///The custom fields will only be altered when they are included in a request. Sending an empty array will not remove any existing values. To remove a field send the name with a null or empty value.
    pub custom_fields: Option<CustomFields>,
    ///Accounting code for invoice line items.
    pub accounting_code: Option<String>,
}
impl std::fmt::Display for ItemUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubscriptionRampInterval {
    ///Represents the billing cycle where a ramp interval starts.
    pub starting_billing_cycle: Option<i64>,
    ///Represents the price for the ramp interval.
    pub unit_amount: Option<i64>,
}
impl std::fmt::Display for SubscriptionRampInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UniqueCouponCode {
    pub id: Option<String>,
    pub updated_at: Option<String>,
    ///The date and time the unique coupon code was redeemed.
    pub redeemed_at: Option<String>,
    pub object: Option<String>,
    ///The Coupon ID of the parent Bulk Coupon
    pub bulk_coupon_id: Option<String>,
    ///The code the customer enters to redeem the coupon.
    pub code: Option<String>,
    ///Indicates if the unique coupon code is redeemable or why not.
    pub state: Option<String>,
    ///The Coupon code of the parent Bulk Coupon
    pub bulk_coupon_code: Option<String>,
    ///The date and time the coupon was expired early or reached its `max_redemptions`.
    pub expired_at: Option<String>,
    pub created_at: Option<String>,
}
impl std::fmt::Display for UniqueCouponCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomFieldDefinitionList {
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    ///Will always be List.
    pub object: Option<String>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
    pub data: Option<Vec<CustomFieldDefinition>>,
}
impl std::fmt::Display for CustomFieldDefinitionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubscriptionList {
    ///Path to subsequent page of results.
    pub next: Option<String>,
    pub data: Option<Vec<Subscription>>,
    ///Will always be List.
    pub object: Option<String>,
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
}
impl std::fmt::Display for SubscriptionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub created_at: Option<String>,
    pub billing_address: Option<AddressWithName>,
    pub object: Option<String>,
    pub ip_address_country: Option<String>,
    ///Time, in seconds, for gateway to process the transaction.
    pub gateway_response_time: Option<f64>,
    pub customer_message_locale: Option<String>,
    ///The method by which the payment was collected.
    pub collection_method: Option<String>,
    ///The UUID is useful for matching data with the CSV exports and building URLs into Recurly's UI.
    pub uuid: Option<String>,
    pub gateway_approval_code: Option<String>,
    ///When processed, result from checking the CVV/CVC value on the transaction.
    pub cvv_check: Option<String>,
    ///Describes how the transaction was triggered.
    pub origin: Option<String>,
    ///Transaction reference number from the payment gateway.
    pub gateway_reference: Option<String>,
    pub account: Option<AccountMini>,
    pub gateway_response_code: Option<String>,
    ///The values in this field will vary from gateway to gateway.
    pub gateway_response_values: Option<serde_json::Value>,
    ///For declined (`success=false`) transactions, the message displayed to the customer.
    pub customer_message: Option<String>,
    ///When processed, result from checking the overall AVS on the transaction.
    pub avs_check: Option<String>,
    pub status_code: Option<String>,
    pub updated_at: Option<String>,
    ///Indicates if part or all of this transaction was refunded.
    pub refunded: Option<bool>,
    pub voided_by_invoice: Option<InvoiceMini>,
    pub payment_gateway: Option<serde_json::Value>,
    ///The current transaction status. Note that the status may change, e.g. a `pending` transaction may become `declined` or `success` may later become `void`.
    pub status: Option<String>,
    pub collected_at: Option<String>,
    ///For declined (`success=false`) transactions, the message displayed to the merchant.
    pub status_message: Option<String>,
    ///3-letter ISO 4217 currency code.
    pub currency: Option<String>,
    pub voided_at: Option<String>,
    pub id: Option<String>,
    /**IP address provided when the billing information was collected:

- When the customer enters billing information into the Recurly.js or Hosted Payment Pages, Recurly records the IP address.
- When the merchant enters billing information using the API, the merchant may provide an IP address.
- When the merchant enters billing information using the UI, no IP address is recorded.
*/
    pub ip_address_v4: Option<String>,
    pub payment_method: Option<PaymentMethod>,
    ///If this transaction is a refund (`type=refund`), this will be the ID of the original transaction on the invoice being refunded.
    pub original_transaction_id: Option<String>,
    ///Indicates if the transaction was completed using a backup payment
    pub backup_payment_method_used: Option<bool>,
    ///Total transaction amount sent to the payment gateway.
    pub amount: Option<f64>,
    ///Transaction message from the payment gateway.
    pub gateway_message: Option<String>,
    ///If the transaction is charging or refunding for one or more subscriptions, these are their IDs.
    pub subscription_ids: Option<Vec<String>>,
    pub invoice: Option<InvoiceMini>,
    /**- `authorization` – verifies billing information and places a hold on money in the customer's account.
- `capture` – captures funds held by an authorization and completes a purchase.
- `purchase` – combines the authorization and capture in one transaction.
- `refund` – returns all or a portion of the money collected in a previous transaction to the customer.
- `verify` – a $0 or $1 transaction used to verify billing information which is immediately voided.
*/
    pub type_: Option<String>,
    ///Did this transaction complete successfully?
    pub success: Option<bool>,
}
impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BillingInfoVerifyCvv {
    ///Unique security code for a credit card.
    pub verification_value: Option<String>,
}
impl std::fmt::Display for BillingInfoVerifyCvv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AccountBalanceAmount {
    ///Total amount the account is past due.
    pub amount: Option<f64>,
    ///3-letter ISO 4217 currency code.
    pub currency: Option<String>,
    ///Total amount for the prepayment credit invoices in a `processing` state on the account.
    pub processing_prepayment_amount: Option<f64>,
    ///Total amount of the open balances on credit invoices for the account.
    pub available_credit_amount: Option<f64>,
}
impl std::fmt::Display for AccountBalanceAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlanCreate {
    ///Unique code to identify the plan. This is used in Hosted Payment Page URLs and in the invoice exports.
    pub code: String,
    ///Accounting code for invoice line items for the plan. If no value is provided, it defaults to plan's code.
    pub accounting_code: Option<String>,
    pub currencies: Vec<PlanPricing>,
    pub add_ons: Option<Vec<AddOnCreate>>,
    ///This name describes your plan and will appear on the Hosted Payment Page and the subscriber's invoice.
    pub name: String,
    ///Automatically terminate plans after a defined number of billing cycles.
    pub total_billing_cycles: Option<i64>,
    ///Allow free trial subscriptions to be created without billing info. Should not be used if billing info is needed for initial invoice due to existing uninvoiced charges or setup fee.
    pub trial_requires_billing_info: Option<bool>,
    ///Units for the plan's trial period.
    pub trial_unit: Option<String>,
    pub setup_fee_revenue_schedule_type: Option<String>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the plan is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types.
    pub avalara_transaction_type: Option<i64>,
    /**Used to determine whether items can be assigned as add-ons to individual subscriptions.
If `true`, items can be assigned as add-ons to individual subscription add-ons.
If `false`, only plan add-ons can be used.
*/
    pub allow_any_item_on_subscriptions: Option<bool>,
    ///Unit for the plan's billing interval.
    pub interval_unit: Option<String>,
    ///Length of plan's trial period in `trial_units`. `0` means `no trial`.
    pub trial_length: Option<i64>,
    ///Subscriptions will automatically inherit this value once they are active. If `auto_renew` is `true`, then a subscription will automatically renew its term at renewal. If `auto_renew` is `false`, then a subscription will expire at the end of its term. `auto_renew` can be overridden on the subscription record itself.
    pub auto_renew: Option<bool>,
    pub ramp_intervals: Option<Vec<PlanRampInterval>>,
    ///Length of the plan's billing interval in `interval_unit`.
    pub interval_length: Option<i64>,
    ///The custom fields will only be altered when they are included in a request. Sending an empty array will not remove any existing values. To remove a field send the name with a null or empty value.
    pub custom_fields: Option<CustomFields>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the plan is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types.
    pub avalara_service_type: Option<i64>,
    ///Optional description, not displayed.
    pub description: Option<String>,
    ///Optional field used by Avalara, Vertex, and Recurly's EU VAT tax feature to determine taxation rules. If you have your own AvaTax or Vertex account configured, use their tax codes to assign specific tax rules. If you are using Recurly's EU VAT feature, you can use values of `unknown`, `physical`, or `digital`.
    pub tax_code: Option<String>,
    pub hosted_pages: Option<PlanHostedPages>,
    ///`true` exempts tax on the plan, `false` applies tax on the plan.
    pub tax_exempt: Option<bool>,
    pub revenue_schedule_type: Option<String>,
    ///Accounting code for invoice line items for the plan's setup fee. If no value is provided, it defaults to plan's accounting code.
    pub setup_fee_accounting_code: Option<String>,
    ///Unique ID to identify a dunning campaign. Used to specify if a non-default dunning campaign should be assigned to this plan. For sites without multiple dunning campaigns enabled, the default dunning campaign will always be used.
    pub dunning_campaign_id: Option<String>,
    /**A fixed pricing model has the same price for each billing period.
A ramp pricing model defines a set of Ramp Intervals, where a subscription changes price on
a specified cadence of billing periods. The price change could be an increase or decrease.
*/
    pub pricing_model: Option<String>,
}
impl std::fmt::Display for PlanCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShippingAddressCreate {
    pub city: String,
    pub company: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub street1: String,
    pub last_name: String,
    pub nickname: Option<String>,
    pub first_name: String,
    ///State or province.
    pub region: Option<String>,
    pub vat_number: Option<String>,
    pub street2: Option<String>,
    ///Zip or postal code.
    pub postal_code: String,
    ///Country, 2-letter ISO 3166-1 alpha-2 code.
    pub country: String,
}
impl std::fmt::Display for ShippingAddressCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateLineItemRequired {
    pub currency: String,
    pub type_: String,
    pub unit_amount: f64,
    pub account_id: String,
}
impl std::fmt::Display for CreateLineItemRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CouponBulkCreate {
    ///The quantity of unique coupon codes to generate
    pub number_of_unique_codes: Option<i64>,
}
impl std::fmt::Display for CouponBulkCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MeasuredUnitUpdate {
    ///Display name for the measured unit.
    pub display_name: Option<String>,
    ///Optional internal description.
    pub description: Option<String>,
    ///Unique internal name of the measured unit on your site.
    pub name: Option<String>,
}
impl std::fmt::Display for MeasuredUnitUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionCreate {
    pub collection_method: Option<String>,
    ///If there are pending credits on the account that will be invoiced during the subscription creation, these will be used as the Customer Notes on the credit invoice.
    pub credit_customer_notes: Option<String>,
    ///For manual invoicing, this identifies the PO number associated with the subscription.
    pub po_number: Option<String>,
    ///Integer representing the number of days after an invoice's creation that the invoice will become past due. If an invoice's net terms are set to '0', it is due 'On Receipt' and will become past due 24 hours after it’s created. If an invoice is due net 30, it will become past due at 31 days exactly.
    pub net_terms: Option<i64>,
    ///Optionally override the default quantity of 1.
    pub quantity: Option<i64>,
    pub revenue_schedule_type: Option<String>,
    ///The new set of ramp intervals for the subscription.
    pub ramp_intervals: Option<Vec<SubscriptionRampInterval>>,
    pub shipping: Option<SubscriptionShippingCreate>,
    ///The `billing_info_id` is the value that represents a specific billing info for an end customer. When `billing_info_id` is used to assign billing info to the subscription, all future billing events for the subscription will bill to the specified billing info. `billing_info_id` can ONLY be used for sites utilizing the Wallet feature.
    pub billing_info_id: Option<String>,
    ///If set, the subscription will begin in the future on this date. The subscription will apply the setup fee and trial period, unless the plan has no trial.
    pub starts_at: Option<String>,
    ///If present, this sets the date the subscription's next billing period will start (`current_period_ends_at`). This can be used to align the subscription’s billing to a specific day of the month. The initial invoice will be prorated for the period between the subscription's activation date and the billing period end date. Subsequent periods will be based off the plan interval. For a subscription with a trial period, this will change when the trial expires.
    pub next_bill_date: Option<String>,
    ///This will default to the Terms and Conditions text specified on the Invoice Settings page in your Recurly admin. Specify custom notes to add or override Terms and Conditions. Custom notes will stay with a subscription on all renewals.
    pub terms_and_conditions: Option<String>,
    ///An optional type designation for the payment gateway transaction created by this request. Supports 'moto' value, which is the acronym for mail order and telephone transactions.
    pub transaction_type: Option<String>,
    ///If `auto_renew=true`, when a term completes, `total_billing_cycles` takes this value as the length of subsequent terms. Defaults to the plan's `total_billing_cycles`.
    pub renewal_billing_cycles: Option<i64>,
    ///Determines whether or not tax is included in the unit amount. The Tax Inclusive Pricing feature (separate from the Mixed Tax Pricing feature) must be enabled to use this flag.
    pub tax_inclusive: Option<bool>,
    ///If set, overrides the default trial behavior for the subscription. When the current date time or a past date time is provided the subscription will begin with no trial phase (overriding any plan default trial). When a future date time is provided the subscription will begin with a trial phase ending at the specified date time.
    pub trial_ends_at: Option<String>,
    ///You must provide either a `plan_code` or `plan_id`. If both are provided the `plan_id` will be used.
    pub plan_code: String,
    ///You must provide either a `plan_code` or `plan_id`. If both are provided the `plan_id` will be used.
    pub plan_id: Option<String>,
    pub account: AccountCreate,
    ///Override the unit amount of the subscription plan by setting this value. If not provided, the subscription will inherit the price from the subscription plan for the provided currency.
    pub unit_amount: Option<f64>,
    ///The number of cycles/billing periods in a term. When `remaining_billing_cycles=0`, if `auto_renew=true` the subscription will renew and a new term will begin, otherwise the subscription will expire.
    pub total_billing_cycles: Option<i64>,
    pub add_ons: Option<Vec<SubscriptionAddOnCreate>>,
    ///A list of coupon_codes to be redeemed on the subscription or account during the purchase.
    pub coupon_codes: Option<Vec<String>>,
    ///Whether the subscription renews at the end of its term.
    pub auto_renew: Option<bool>,
    ///This will default to the Customer Notes text specified on the Invoice Settings. Specify custom notes to add or override Customer Notes. Custom notes will stay with a subscription on all renewals.
    pub customer_notes: Option<String>,
    ///The custom fields will only be altered when they are included in a request. Sending an empty array will not remove any existing values. To remove a field send the name with a null or empty value.
    pub custom_fields: Option<CustomFields>,
    ///3-letter ISO 4217 currency code.
    pub currency: String,
}
impl std::fmt::Display for SubscriptionCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LineItemList {
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    pub data: Option<Vec<LineItem>>,
    ///Will always be List.
    pub object: Option<String>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
}
impl std::fmt::Display for LineItemList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionChangeShippingCreate {
    ///Assign a shipping address from the account's existing shipping addresses. If this and address are both present, address will take precedence.
    pub address_id: Option<String>,
    pub address: Option<ShippingAddressCreate>,
    ///The code of the shipping method used to deliver the subscription. To remove shipping set this to `null` and the `amount=0`. If `method_id` and `method_code` are both present, `method_id` will be used.
    pub method_code: Option<String>,
    ///The id of the shipping method used to deliver the subscription. To remove shipping set this to `null` and the `amount=0`. If `method_id` and `method_code` are both present, `method_id` will be used.
    pub method_id: Option<String>,
    pub amount: Option<f64>,
}
impl std::fmt::Display for SubscriptionChangeShippingCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UsageList {
    ///Will always be List.
    pub object: Option<String>,
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
    pub data: Option<Vec<Usage>>,
}
impl std::fmt::Display for UsageList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    /**- full:      Full Address (Street, City, State, Postal Code and Country)
- streetzip: Street and Postal Code only
- zip:       Postal Code only
- none:      No Address
*/
    pub billing_address_requirement: Option<String>,
    pub accepted_currencies: Option<Vec<String>>,
    ///The default 3-letter ISO 4217 currency code.
    pub default_currency: Option<String>,
}
impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub account_read_only: AccountReadOnly,
    pub account_response: AccountResponse,
}
impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlanUpdate {
    ///Accounting code for invoice line items for the plan. If no value is provided, it defaults to plan's code.
    pub accounting_code: Option<String>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the plan is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types.
    pub avalara_service_type: Option<i64>,
    ///Optional when the pricing model is 'ramp'.
    pub currencies: Option<Vec<PlanPricing>>,
    pub hosted_pages: Option<PlanHostedPages>,
    pub id: Option<String>,
    ///Unique code to identify the plan. This is used in Hosted Payment Page URLs and in the invoice exports.
    pub code: Option<String>,
    ///Allow free trial subscriptions to be created without billing info. Should not be used if billing info is needed for initial invoice due to existing uninvoiced charges or setup fee.
    pub trial_requires_billing_info: Option<bool>,
    /**Used to determine whether items can be assigned as add-ons to individual subscriptions.
If `true`, items can be assigned as add-ons to individual subscription add-ons.
If `false`, only plan add-ons can be used.
*/
    pub allow_any_item_on_subscriptions: Option<bool>,
    ///Units for the plan's trial period.
    pub trial_unit: Option<String>,
    ///Subscriptions will automatically inherit this value once they are active. If `auto_renew` is `true`, then a subscription will automatically renew its term at renewal. If `auto_renew` is `false`, then a subscription will expire at the end of its term. `auto_renew` can be overridden on the subscription record itself.
    pub auto_renew: Option<bool>,
    ///Optional description, not displayed.
    pub description: Option<String>,
    ///Accounting code for invoice line items for the plan's setup fee. If no value is provided, it defaults to plan's accounting code.
    pub setup_fee_accounting_code: Option<String>,
    ///Used by Avalara for Communications taxes. The transaction type in combination with the service type describe how the plan is taxed. Refer to [the documentation](https://help.avalara.com/AvaTax_for_Communications/Tax_Calculation/AvaTax_for_Communications_Tax_Engine/Mapping_Resources/TM_00115_AFC_Modules_Corresponding_Transaction_Types) for more available t/s types.
    pub avalara_transaction_type: Option<i64>,
    pub revenue_schedule_type: Option<String>,
    pub setup_fee_revenue_schedule_type: Option<String>,
    ///Optional field used by Avalara, Vertex, and Recurly's EU VAT tax feature to determine taxation rules. If you have your own AvaTax or Vertex account configured, use their tax codes to assign specific tax rules. If you are using Recurly's EU VAT feature, you can use values of `unknown`, `physical`, or `digital`.
    pub tax_code: Option<String>,
    ///Length of plan's trial period in `trial_units`. `0` means `no trial`.
    pub trial_length: Option<i64>,
    ///`true` exempts tax on the plan, `false` applies tax on the plan.
    pub tax_exempt: Option<bool>,
    ///Unique ID to identify a dunning campaign. Used to specify if a non-default dunning campaign should be assigned to this plan. For sites without multiple dunning campaigns enabled, the default dunning campaign will always be used.
    pub dunning_campaign_id: Option<String>,
    ///This name describes your plan and will appear on the Hosted Payment Page and the subscriber's invoice.
    pub name: Option<String>,
    ///Automatically terminate plans after a defined number of billing cycles.
    pub total_billing_cycles: Option<i64>,
    ///The custom fields will only be altered when they are included in a request. Sending an empty array will not remove any existing values. To remove a field send the name with a null or empty value.
    pub custom_fields: Option<CustomFields>,
    pub ramp_intervals: Option<Vec<PlanRampInterval>>,
}
impl std::fmt::Display for PlanUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceCollection {
    pub object: Option<String>,
    pub charge_invoice: Option<Invoice>,
    pub credit_invoices: Option<Vec<Invoice>>,
}
impl std::fmt::Display for InvoiceCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionPurchase {
    ///Determines whether or not tax is included in the unit amount. The Tax Inclusive Pricing feature (separate from the Mixed Tax Pricing feature) must be enabled to use this flag.
    pub tax_inclusive: Option<bool>,
    ///If `auto_renew=true`, when a term completes, `total_billing_cycles` takes this value as the length of subsequent terms. Defaults to the plan's `total_billing_cycles`.
    pub renewal_billing_cycles: Option<i64>,
    ///Optionally override the default quantity of 1.
    pub quantity: Option<i64>,
    ///The number of cycles/billing periods in a term. When `remaining_billing_cycles=0`, if `auto_renew=true` the subscription will renew and a new term will begin, otherwise the subscription will expire.
    pub total_billing_cycles: Option<i64>,
    pub plan_code: String,
    ///Override the unit amount of the subscription plan by setting this value. If not provided, the subscription will inherit the price from the subscription plan for the provided currency.
    pub unit_amount: Option<f64>,
    pub shipping: Option<SubscriptionShippingPurchase>,
    ///If set, overrides the default trial behavior for the subscription. When the current date time or a past date time is provided the subscription will begin with no trial phase (overriding any plan default trial). When a future date time is provided the subscription will begin with a trial phase ending at the specified date time.
    pub trial_ends_at: Option<String>,
    ///If present, this sets the date the subscription's next billing period will start (`current_period_ends_at`). This can be used to align the subscription’s billing to a specific day of the month. The initial invoice will be prorated for the period between the subscription's activation date and the billing period end date. Subsequent periods will be based off the plan interval. For a subscription with a trial period, this will change when the trial expires.
    pub next_bill_date: Option<String>,
    ///Whether the subscription renews at the end of its term.
    pub auto_renew: Option<bool>,
    pub revenue_schedule_type: Option<String>,
    ///The custom fields will only be altered when they are included in a request. Sending an empty array will not remove any existing values. To remove a field send the name with a null or empty value.
    pub custom_fields: Option<CustomFields>,
    ///If set, the subscription will begin in the future on this date. The subscription will apply the setup fee and trial period, unless the plan has no trial.
    pub starts_at: Option<String>,
    ///The new set of ramp intervals for the subscription.
    pub ramp_intervals: Option<Vec<SubscriptionRampInterval>>,
    pub add_ons: Option<Vec<SubscriptionAddOnCreate>>,
    pub plan_id: Option<String>,
}
impl std::fmt::Display for SubscriptionPurchase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
    pub id: Option<String>,
    pub time_zone: Option<String>,
    pub deleted_at: Option<String>,
    pub created_at: Option<String>,
    pub first_name: Option<String>,
    pub object: Option<String>,
    pub email: Option<String>,
    pub last_name: Option<String>,
}
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InvoiceAddress {
    pub name_on_account: Option<String>,
    pub company: Option<String>,
}
impl std::fmt::Display for InvoiceAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomerPermission {
    ///Customer permission code.
    pub code: Option<String>,
    ///It will always be "customer_permission".
    pub object: Option<String>,
    ///Customer permission ID.
    pub id: Option<String>,
    ///Customer permission name.
    pub name: Option<String>,
    ///Description of customer permission.
    pub description: Option<String>,
}
impl std::fmt::Display for CustomerPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateShippingAddressRequired {
    pub postal_code: String,
    pub first_name: String,
    pub country: String,
    pub street1: String,
    pub city: String,
    pub account_id: String,
    pub last_name: String,
}
impl std::fmt::Display for CreateShippingAddressRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CouponRedemptionList {
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    pub data: Option<Vec<CouponRedemption>>,
    ///Will always be List.
    pub object: Option<String>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
}
impl std::fmt::Display for CouponRedemptionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AddressWithName {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
impl std::fmt::Display for AddressWithName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBalance {
    pub past_due: Option<bool>,
    pub account: Option<AccountMini>,
    pub balances: Option<Vec<AccountBalanceAmount>>,
    pub object: Option<String>,
}
impl std::fmt::Display for AccountBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionAddOn {
    ///Supports up to 2 decimal places.
    pub unit_amount: Option<f64>,
    pub created_at: Option<String>,
    pub id: Option<String>,
    /**The pricing model for the add-on.  For more information,
[click here](https://docs.recurly.com/docs/billing-models#section-quantity-based). See our
[Guide](https://recurly.com/developers/guides/item-addon-guide.html) for an overview of how
to configure quantity-based pricing models.
*/
    pub tier_type: Option<String>,
    ///Just the important parts.
    pub add_on: Option<AddOnMini>,
    pub revenue_schedule_type: Option<String>,
    pub subscription_id: Option<String>,
    pub updated_at: Option<String>,
    pub quantity: Option<i64>,
    ///Supports up to 9 decimal places.
    pub unit_amount_decimal: Option<String>,
    /**Used to determine where the associated add-on data is pulled from. If this value is set to
`plan_add_on` or left blank, then add-on data will be pulled from the plan's add-ons. If the associated
`plan` has `allow_any_item_on_subscriptions` set to `true` and this field is set to `item`, then
the associated add-on data will be pulled from the site's item catalog.
*/
    pub add_on_source: Option<String>,
    /**If tiers are provided in the request, all existing tiers on the Subscription Add-on will be
removed and replaced by the tiers in the request. If add_on.tier_type is tiered or volume and
add_on.usage_type is percentage use percentage_tiers instead.
There must be one tier without an `ending_quantity` value which represents the final tier.
*/
    pub tiers: Option<Vec<SubscriptionAddOnTier>>,
    /**If percentage tiers are provided in the request, all existing percentage tiers on the Subscription Add-on will be
removed and replaced by the percentage tiers in the request. Use only if add_on.tier_type is tiered or volume and
add_on.usage_type is percentage.
There must be one tier without an `ending_amount` value which represents the final tier.
*/
    pub percentage_tiers: Option<Vec<SubscriptionAddOnPercentageTier>>,
    ///The time at which usage totals are reset for billing purposes.
    pub usage_timeframe: Option<String>,
    ///The percentage taken of the monetary amount of usage tracked. This can be up to 4 decimal places. A value between 0.0 and 100.0. Required if add_on_type is usage and usage_type is percentage.
    pub usage_percentage: Option<f64>,
    pub expired_at: Option<String>,
    pub object: Option<String>,
    ///The type of calculation to be employed for an add-on.  Cumulative billing will sum all usage records created in the current billing cycle.  Last-in-period billing will apply only the most recent usage record in the billing period.  If no value is specified, cumulative billing will be used.
    pub usage_calculation_type: Option<String>,
}
impl std::fmt::Display for SubscriptionAddOn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountAcquisitionUpdate {
    ///The channel through which the account was acquired.
    pub channel: Option<String>,
    ///An arbitrary subchannel string representing a distinction/subcategory within a broader channel.
    pub subchannel: Option<String>,
    pub cost: Option<serde_json::Value>,
    ///An arbitrary identifier for the marketing campaign that led to the acquisition of this account.
    pub campaign: Option<String>,
}
impl std::fmt::Display for AccountAcquisitionUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Address {
    ///State or province.
    pub region: Option<String>,
    pub street2: Option<String>,
    ///Zip or postal code.
    pub postal_code: Option<String>,
    pub street1: Option<String>,
    ///Country, 2-letter ISO 3166-1 alpha-2 code.
    pub country: Option<String>,
    pub city: Option<String>,
    pub phone: Option<String>,
}
impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CouponMini {
    ///The code the customer enters to redeem the coupon.
    pub code: Option<String>,
    ///Indicates if the coupon is redeemable, and if it is not, why.
    pub state: Option<String>,
    ///The date and time the coupon was expired early or reached its `max_redemptions`.
    pub expired_at: Option<String>,
    pub id: Option<String>,
    ///Whether the coupon is "single_code" or "bulk". Bulk coupons will require a `unique_code_template` and will generate unique codes through the `/generate` endpoint.
    pub coupon_type: Option<String>,
    /**Details of the discount a coupon applies. Will contain a `type`
property and one of the following properties: `percent`, `fixed`, `trial`.
*/
    pub discount: Option<CouponDiscount>,
    pub object: Option<String>,
    ///The internal name for the coupon.
    pub name: Option<String>,
}
impl std::fmt::Display for CouponMini {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryFile(pub String);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InvoiceTemplate {
    ///Invoice template description.
    pub description: Option<String>,
    ///When the invoice template was created in Recurly.
    pub created_at: Option<String>,
    ///When the invoice template was updated in Recurly.
    pub updated_at: Option<String>,
    pub id: Option<String>,
    ///Invoice template code.
    pub code: Option<String>,
    ///Invoice template name.
    pub name: Option<String>,
}
impl std::fmt::Display for InvoiceTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlanRampInterval {
    ///Represents the billing cycle where a ramp interval starts.
    pub starting_billing_cycle: Option<i64>,
    ///Represents the price for the ramp interval.
    pub currencies: Option<Vec<PlanRampPricing>>,
}
impl std::fmt::Display for PlanRampInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub revenue_schedule_type: Option<String>,
    pub expiration_reason: Option<String>,
    pub object: Option<String>,
    pub account: Option<AccountMini>,
    pub trial_started_at: Option<String>,
    ///Just the important parts.
    pub plan: Option<PlanMini>,
    ///Null unless subscription is paused or will pause at the end of the current billing period.
    pub paused_at: Option<String>,
    ///3-letter ISO 4217 currency code.
    pub currency: Option<String>,
    pub current_period_started_at: Option<String>,
    ///The number of cycles/billing periods in a term. When `remaining_billing_cycles=0`, if `auto_renew=true` the subscription will renew and a new term will begin, otherwise the subscription will expire.
    pub total_billing_cycles: Option<i64>,
    ///If `auto_renew=true`, when a term completes, `total_billing_cycles` takes this value as the length of subsequent terms. Defaults to the plan's `total_billing_cycles`.
    pub renewal_billing_cycles: Option<i64>,
    pub pending_change: Option<SubscriptionChange>,
    pub current_period_ends_at: Option<String>,
    pub shipping: Option<SubscriptionShipping>,
    pub tax: Option<f64>,
    pub terms_and_conditions: Option<String>,
    pub customer_notes: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    ///Recurring subscriptions paid with ACH will have this attribute set. This timestamp is used for alerting customers to reauthorize in 3 years in accordance with NACHA rules. If a subscription becomes inactive or the billing info is no longer a bank account, this timestamp is cleared.
    pub bank_account_authorized_at: Option<String>,
    ///Whether the subscription renews at the end of its term.
    pub auto_renew: Option<bool>,
    pub trial_ends_at: Option<String>,
    ///The ramp intervals representing the pricing schedule for the subscription.
    pub ramp_intervals: Option<Vec<SubscriptionRampIntervalResponse>>,
    pub subtotal: Option<f64>,
    pub tax_info: Option<TaxInfo>,
    ///Billing Info ID.
    pub billing_info_id: Option<String>,
    ///For manual invoicing, this identifies the PO number associated with the subscription.
    pub po_number: Option<String>,
    pub add_ons_total: Option<f64>,
    ///Returns subscription level coupon redemptions that are tied to this subscription.
    pub coupon_redemptions: Option<Vec<CouponRedemptionMini>>,
    pub activated_at: Option<String>,
    pub id: Option<String>,
    pub expires_at: Option<String>,
    pub add_ons: Option<Vec<SubscriptionAddOn>>,
    ///The invoice ID of the latest invoice created for an active subscription.
    pub active_invoice_id: Option<String>,
    ///The remaining billing cycles in the current term.
    pub remaining_billing_cycles: Option<i64>,
    ///Null unless subscription is paused or will pause at the end of the current billing period.
    pub remaining_pause_cycles: Option<i64>,
    pub unit_amount: Option<f64>,
    ///Determines whether or not tax is included in the unit amount. The Tax Inclusive Pricing feature (separate from the Mixed Tax Pricing feature) must be enabled to utilize this flag.
    pub tax_inclusive: Option<bool>,
    ///Integer representing the number of days after an invoice's creation that the invoice will become past due. If an invoice's net terms are set to '0', it is due 'On Receipt' and will become past due 24 hours after it’s created. If an invoice is due net 30, it will become past due at 31 days exactly.
    pub net_terms: Option<i64>,
    ///The custom fields will only be altered when they are included in a request. Sending an empty array will not remove any existing values. To remove a field send the name with a null or empty value.
    pub custom_fields: Option<CustomFields>,
    pub collection_method: Option<String>,
    pub state: Option<String>,
    ///If present, this subscription's transactions will use the payment gateway with this code.
    pub gateway_code: Option<String>,
    ///When the term ends. This is calculated by a plan's interval and `total_billing_cycles` in a term. Subscription changes with a `timeframe=renewal` will be applied on this date.
    pub current_term_ends_at: Option<String>,
    pub total: Option<f64>,
    ///The UUID is useful for matching data with the CSV exports and building URLs into Recurly's UI.
    pub uuid: Option<String>,
    pub quantity: Option<i64>,
    ///The start date of the term when the first billing period starts. The subscription term is the length of time that a customer will be committed to a subscription. A term can span multiple billing periods.
    pub current_term_started_at: Option<String>,
    pub canceled_at: Option<String>,
}
impl std::fmt::Display for Subscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CouponRedemption {
    pub state: Option<String>,
    pub updated_at: Option<String>,
    pub coupon: Option<Coupon>,
    pub created_at: Option<String>,
    ///3-letter ISO 4217 currency code.
    pub currency: Option<String>,
    pub id: Option<String>,
    ///The amount that was discounted upon the application of the coupon, formatted with the currency.
    pub discounted: Option<f64>,
    ///The date and time the redemption was removed from the account (un-redeemed).
    pub removed_at: Option<String>,
    ///Will always be `coupon`.
    pub object: Option<String>,
    pub subscription_id: Option<String>,
    pub account: Option<AccountMini>,
}
impl std::fmt::Display for CouponRedemption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlanPricing {
    ///3-letter ISO 4217 currency code.
    pub currency: Option<String>,
    ///This field should not be sent when the pricing model is 'ramp'.
    pub unit_amount: Option<f64>,
    ///This field is deprecated. Please do not use it.
    pub tax_inclusive: Option<bool>,
    ///Amount of one-time setup fee automatically charged at the beginning of a subscription billing cycle. For subscription plans with a trial, the setup fee will be charged at the time of signup. Setup fees do not increase with the quantity of a subscription plan.
    pub setup_fee: Option<f64>,
}
impl std::fmt::Display for PlanPricing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DunningCycle {
    ///When the current settings were updated in Recurly.
    pub updated_at: Option<String>,
    ///Dunning intervals.
    pub intervals: Option<Vec<DunningInterval>>,
    ///Whether the dunning settings will be applied to manual trials. Only applies to trial cycles.
    pub applies_to_manual_trial: Option<bool>,
    ///When the current settings were created in Recurly.
    pub created_at: Option<String>,
    ///The number of days between the first dunning email being sent and the end of the dunning cycle.
    pub total_dunning_days: Option<i64>,
    ///The type of invoice this cycle applies to.
    pub type_: Option<String>,
    ///Whether or not to send an extra email immediately to customers whose initial payment attempt fails with either a hard decline or invalid billing info.
    pub send_immediately_on_hard_decline: Option<bool>,
    ///Whether the subscription(s) should be cancelled at the end of the dunning cycle.
    pub expire_subscription: Option<bool>,
    ///The number of days after a transaction failure before the first dunning email is sent.
    pub first_communication_interval: Option<i64>,
    ///Whether the invoice should be failed at the end of the dunning cycle.
    pub fail_invoice: Option<bool>,
    ///The number of days between a transaction failure and the end of the dunning cycle.
    pub total_recycling_days: Option<i64>,
    ///Current campaign version.
    pub version: Option<i64>,
}
impl std::fmt::Display for DunningCycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CouponPricing {
    ///The fixed discount (in dollars) for the corresponding currency.
    pub discount: Option<f64>,
    ///3-letter ISO 4217 currency code.
    pub currency: Option<String>,
}
impl std::fmt::Display for CouponPricing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShippingAddressList {
    ///Path to subsequent page of results.
    pub next: Option<String>,
    ///Will always be List.
    pub object: Option<String>,
    pub data: Option<Vec<ShippingAddress>>,
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
}
impl std::fmt::Display for ShippingAddressList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AddOnMini {
    ///Whether the add-on type is fixed, or usage-based.
    pub add_on_type: Option<String>,
    ///The unique identifier for the add-on within its plan.
    pub code: Option<String>,
    ///Describes your add-on and will appear in subscribers' invoices.
    pub name: Option<String>,
    ///Type of usage, returns usage type if `add_on_type` is `usage`.
    pub usage_type: Option<String>,
    pub object: Option<String>,
    ///The percentage taken of the monetary amount of usage tracked. This can be up to 4 decimal places. A value between 0.0 and 100.0.
    pub usage_percentage: Option<f64>,
    pub item_id: Option<String>,
    ///Accounting code for invoice line items for this add-on. If no value is provided, it defaults to add-on's code.
    pub accounting_code: Option<String>,
    ///Optional, stock keeping unit to link the item to other inventory systems.
    pub external_sku: Option<String>,
    pub id: Option<String>,
    ///System-generated unique identifier for an measured unit associated with the add-on.
    pub measured_unit_id: Option<String>,
}
impl std::fmt::Display for AddOnMini {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountCreate {
    pub account_update: AccountUpdate,
    pub acquisition: AccountAcquisitionUpdate,
    ///The unique identifier of the account. This cannot be changed once the account is created.
    pub code: String,
    pub shipping_addresses: Vec<ShippingAddressCreate>,
}
impl std::fmt::Display for AccountCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TaxDetail {
    ///Provides the tax rate for the region.
    pub rate: Option<f64>,
    ///Provides the tax type for the region. For Canadian Sales Tax, this will be GST, HST, QST or PST.
    pub type_: Option<String>,
    ///Provides the tax region applied on an invoice. For Canadian Sales Tax, this will be either the 2 letter province code or country code.
    pub region: Option<String>,
    ///The total tax applied for this tax type.
    pub tax: Option<f64>,
}
impl std::fmt::Display for TaxDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tier {
    ///Ending quantity for the tier.  This represents a unit amount for unit-priced add ons. Must be left empty if it is the final tier.
    pub ending_quantity: Option<i64>,
    ///(deprecated) -- Use the percentage_tiers object instead.
    pub usage_percentage: Option<String>,
    pub currencies: Option<Vec<TierPricing>>,
}
impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountUpdate {
    ///The VAT number of the account (to avoid having the VAT applied). This is only used for manually collected invoices.
    pub vat_number: Option<String>,
    ///An optional type designation for the payment gateway transaction created by this request. Supports 'moto' value, which is the acronym for mail order and telephone transactions.
    pub transaction_type: Option<String>,
    ///The email address used for communicating with this customer. The customer will also use this email address to log into your hosted account management pages. This value does not need to be unique.
    pub email: Option<String>,
    ///An enumerable describing the billing behavior of the account, specifically whether the account is self-paying or will rely on the parent account to pay.
    pub bill_to: Option<String>,
    ///The account code of the parent account to be associated with this account. Passing an empty value removes any existing parent association from this account. If both `parent_account_code` and `parent_account_id` are passed, the non-blank value in `parent_account_id` will be used. Only one level of parent child relationship is allowed. You cannot assign a parent account that itself has a parent account.
    pub parent_account_code: Option<String>,
    ///Unique ID to identify a dunning campaign. Used to specify if a non-default dunning campaign should be assigned to this account. For sites without multiple dunning campaigns enabled, the default dunning campaign will always be used.
    pub dunning_campaign_id: Option<String>,
    ///Additional email address that should receive account correspondence. These should be separated only by commas. These CC emails will receive all emails that the `email` field also receives.
    pub cc_emails: Option<String>,
    ///The tax exemption certificate number for the account. If the merchant has an integration for the Vertex tax provider, this optional value will be sent in any tax calculation requests for the account.
    pub exemption_certificate: Option<String>,
    ///The UUID of the parent account to be associated with this account. Passing an empty value removes any existing parent association from this account. If both `parent_account_code` and `parent_account_id` are passed, the non-blank value in `parent_account_id` will be used. Only one level of parent child relationship is allowed. You cannot assign a parent account that itself has a parent account.
    pub parent_account_id: Option<String>,
    ///Unique ID to identify an invoice template.  Available when the site is on a Pro or Enterprise plan.  Used to specify which invoice template, if any, should be used to generate invoices for the account.
    pub invoice_template_id: Option<String>,
    pub address: Option<Address>,
    pub first_name: Option<String>,
    pub billing_info: Option<BillingInfoCreate>,
    ///The custom fields will only be altered when they are included in a request. Sending an empty array will not remove any existing values. To remove a field send the name with a null or empty value.
    pub custom_fields: Option<CustomFields>,
    ///Used to determine the language and locale of emails sent on behalf of the merchant to the customer. The list of locales is restricted to those the merchant has enabled on the site.
    pub preferred_locale: Option<String>,
    ///A secondary value for the account.
    pub username: Option<String>,
    pub last_name: Option<String>,
    pub company: Option<String>,
    ///The tax status of the account. `true` exempts tax on the account, `false` applies tax on the account.
    pub tax_exempt: Option<bool>,
}
impl std::fmt::Display for AccountUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountResponse {
    ///The tax exemption certificate number for the account. If the merchant has an integration for the Vertex tax provider, this optional value will be sent in any tax calculation requests for the account.
    pub exemption_certificate: Option<String>,
    ///A secondary value for the account.
    pub username: Option<String>,
    ///The VAT number of the account (to avoid having the VAT applied). This is only used for manually collected invoices.
    pub vat_number: Option<String>,
    ///The tax status of the account. `true` exempts tax on the account, `false` applies tax on the account.
    pub tax_exempt: Option<bool>,
    ///The email address used for communicating with this customer. The customer will also use this email address to log into your hosted account management pages. This value does not need to be unique.
    pub email: Option<String>,
    ///Unique ID to identify a dunning campaign. Used to specify if a non-default dunning campaign should be assigned to this account. For sites without multiple dunning campaigns enabled, the default dunning campaign will always be used.
    pub dunning_campaign_id: Option<String>,
    ///Used to determine the language and locale of emails sent on behalf of the merchant to the customer.
    pub preferred_locale: Option<String>,
    ///Additional email address that should receive account correspondence. These should be separated only by commas. These CC emails will receive all emails that the `email` field also receives.
    pub cc_emails: Option<String>,
    pub last_name: Option<String>,
    ///An enumerable describing the billing behavior of the account, specifically whether the account is self-paying or will rely on the parent account to pay.
    pub bill_to: Option<String>,
    pub address: Option<Address>,
    pub billing_info: Option<BillingInfo>,
    ///The custom fields will only be altered when they are included in a request. Sending an empty array will not remove any existing values. To remove a field send the name with a null or empty value.
    pub custom_fields: Option<CustomFields>,
    pub first_name: Option<String>,
    ///Unique ID to identify an invoice template. Available when the site is on a Pro or Enterprise plan. Used to specify if a non-default invoice template will be used to generate invoices for the account. For sites without multiple invoice templates enabled, the default template will always be used.
    pub invoice_template_id: Option<String>,
    ///The unique identifier of the account. This cannot be changed once the account is created.
    pub code: Option<String>,
    ///The UUID of the parent account associated with this account.
    pub parent_account_id: Option<String>,
    pub company: Option<String>,
}
impl std::fmt::Display for AccountResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseCreate {
    pub account: AccountPurchase,
    ///Integer representing the number of days after an invoice's creation that the invoice will become past due. If an invoice's net terms are set to '0', it is due 'On Receipt' and will become past due 24 hours after it’s created. If an invoice is due net 30, it will become past due at 31 days exactly.
    pub net_terms: Option<i64>,
    ///Terms and conditions to be put on the purchase invoice.
    pub terms_and_conditions: Option<String>,
    ///A list of one time charges or credits to be created with the purchase.
    pub line_items: Option<Vec<LineItemCreate>>,
    ///For manual invoicing, this identifies the PO number associated with the subscription.
    pub po_number: Option<String>,
    ///3-letter ISO 4217 currency code.
    pub currency: String,
    ///VAT reverse charge notes for cross border European tax settlement.
    pub vat_reverse_charge_notes: Option<String>,
    ///A list of subscriptions to be created with the purchase.
    pub subscriptions: Option<Vec<SubscriptionPurchase>>,
    pub customer_notes: Option<String>,
    ///The `billing_info_id` is the value that represents a specific billing info for an end customer. When `billing_info_id` is used to assign billing info to the subscription, all future billing events for the subscription will bill to the specified billing info. `billing_info_id` can ONLY be used for sites utilizing the Wallet feature.
    pub billing_info_id: Option<String>,
    ///An optional type designation for the payment gateway transaction created by this request. Supports 'moto' value, which is the acronym for mail order and telephone transactions.
    pub transaction_type: Option<String>,
    ///Notes to be put on the credit invoice resulting from credits in the purchase, if any.
    pub credit_customer_notes: Option<String>,
    ///The default payment gateway identifier to be used for the purchase transaction.  This will also be applied as the default for any subscriptions included in the purchase request.
    pub gateway_code: Option<String>,
    ///Must be set to manual in order to preview a purchase for an Account that does not have payment information associated with the Billing Info.
    pub collection_method: Option<String>,
    pub shipping: Option<serde_json::Value>,
    ///A list of coupon_codes to be redeemed on the subscription or account during the purchase.
    pub coupon_codes: Option<Vec<String>>,
    ///A gift card redemption code to be redeemed on the purchase invoice.
    pub gift_card_redemption_code: Option<String>,
}
impl std::fmt::Display for PurchaseCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountAcquisition {
    pub account_acquisition_update: AccountAcquisitionUpdate,
    pub account_acquisition_read_only: AccountAcquisitionReadOnly,
}
impl std::fmt::Display for AccountAcquisition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CouponDiscount {
    ///This is only present when `type=fixed`.
    pub currencies: Option<Vec<CouponDiscountPricing>>,
    ///This is only present when `type=percent`.
    pub percent: Option<i64>,
    pub type_: Option<String>,
    ///This is only present when `type=free_trial`.
    pub trial: Option<serde_json::Value>,
}
impl std::fmt::Display for CouponDiscount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TransactionList {
    ///Indicates there are more results on subsequent pages.
    pub has_more: Option<bool>,
    pub data: Option<Vec<Transaction>>,
    ///Path to subsequent page of results.
    pub next: Option<String>,
    ///Will always be List.
    pub object: Option<String>,
}
impl std::fmt::Display for TransactionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionUpdate {
    pub shipping: Option<SubscriptionShippingUpdate>,
    ///Specify custom notes to add or override Terms and Conditions. Custom notes will stay with a subscription on all renewals.
    pub terms_and_conditions: Option<String>,
    pub revenue_schedule_type: Option<String>,
    ///For manual invoicing, this identifies the PO number associated with the subscription.
    pub po_number: Option<String>,
    ///If present, this subscription's transactions will use the payment gateway with this code.
    pub gateway_code: Option<String>,
    ///The `billing_info_id` is the value that represents a specific billing info for an end customer. When `billing_info_id` is used to assign billing info to the subscription, all future billing events for the subscription will bill to the specified billing info. `billing_info_id` can ONLY be used for sites utilizing the Wallet feature.
    pub billing_info_id: Option<String>,
    ///If `auto_renew=true`, when a term completes, `total_billing_cycles` takes this value as the length of subsequent terms. Defaults to the plan's `total_billing_cycles`.
    pub renewal_billing_cycles: Option<i64>,
    ///The remaining billing cycles in the current term.
    pub remaining_billing_cycles: Option<i64>,
    ///Whether the subscription renews at the end of its term.
    pub auto_renew: Option<bool>,
    ///Specify custom notes to add or override Customer Notes. Custom notes will stay with a subscription on all renewals.
    pub customer_notes: Option<String>,
    ///This field is deprecated. Please do not use it.
    pub tax_inclusive: Option<bool>,
    pub collection_method: Option<String>,
    ///The custom fields will only be altered when they are included in a request. Sending an empty array will not remove any existing values. To remove a field send the name with a null or empty value.
    pub custom_fields: Option<CustomFields>,
    ///Integer representing the number of days after an invoice's creation that the invoice will become past due. If an invoice's net terms are set to '0', it is due 'On Receipt' and will become past due 24 hours after it’s created. If an invoice is due net 30, it will become past due at 31 days exactly.
    pub net_terms: Option<i64>,
    ///If present, this sets the date the subscription's next billing period will start (`current_period_ends_at`). This can be used to align the subscription’s billing to a specific day of the month. For a subscription in a trial period, this will change when the trial expires. This parameter is useful for postponement of a subscription to change its billing date without proration.
    pub next_bill_date: Option<String>,
}
impl std::fmt::Display for SubscriptionUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
