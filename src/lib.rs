//! [`RecurlyClient`](struct.RecurlyClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;

pub struct RecurlyClient {
    pub(crate) client: httpclient::Client,
    authentication: RecurlyAuthentication,
}
impl RecurlyClient {
    pub fn from_env() -> Self {
        let url = std::env::var("RECURLY_BASE_URL")
            .expect("Missing environment variable RECURLY_BASE_URL");
        Self {
            client: httpclient::Client::new(Some(url)),
            authentication: RecurlyAuthentication::from_env(),
        }
    }
}
impl RecurlyClient {
    pub fn new(url: &str, authentication: RecurlyAuthentication) -> Self {
        let client = httpclient::Client::new(Some(url.to_string()));
        Self { client, authentication }
    }
    pub fn with_authentication(mut self, authentication: RecurlyAuthentication) -> Self {
        self.authentication = authentication;
        self
    }
    pub fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {
            RecurlyAuthentication::ApiKey { api_key } => {
                r = r.basic_auth(api_key);
            }
        }
        r
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    /**List sites

This route is most useful for finding a site's ID for subsequent requests.

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.
*/
    pub fn list_sites(&self) -> request::ListSitesRequest {
        request::ListSitesRequest {
            client: &self,
            ids: None,
            limit: None,
            order: None,
            sort: None,
            state: None,
        }
    }
    ///Fetch a site
    pub fn get_site(&self, site_id: &str) -> request::GetSiteRequest {
        request::GetSiteRequest {
            client: &self,
            site_id: site_id.to_owned(),
        }
    }
    /**List a site's accounts

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_accounts(&self) -> request::ListAccountsRequest {
        request::ListAccountsRequest {
            client: &self,
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            email: None,
            subscriber: None,
            past_due: None,
        }
    }
    ///Create an account
    pub fn create_account(
        &self,
        args: request::CreateAccountRequired,
    ) -> request::CreateAccountRequest {
        request::CreateAccountRequest {
            client: &self,
            code: args.code.to_owned(),
            acquisition: args.acquisition,
            shipping_addresses: args.shipping_addresses,
            username: args.username.to_owned(),
            email: args.email.to_owned(),
            preferred_locale: args.preferred_locale.to_owned(),
            cc_emails: args.cc_emails.to_owned(),
            first_name: args.first_name.to_owned(),
            last_name: args.last_name.to_owned(),
            company: args.company.to_owned(),
            vat_number: args.vat_number.to_owned(),
            tax_exempt: args.tax_exempt,
            exemption_certificate: args.exemption_certificate.to_owned(),
            parent_account_code: args.parent_account_code.to_owned(),
            parent_account_id: args.parent_account_id.to_owned(),
            bill_to: args.bill_to.to_owned(),
            transaction_type: args.transaction_type.to_owned(),
            dunning_campaign_id: args.dunning_campaign_id.to_owned(),
            invoice_template_id: args.invoice_template_id.to_owned(),
            address: args.address,
            billing_info: args.billing_info,
            custom_fields: args.custom_fields,
        }
    }
    ///Fetch an account
    pub fn get_account(&self, account_id: &str) -> request::GetAccountRequest {
        request::GetAccountRequest {
            client: &self,
            account_id: account_id.to_owned(),
        }
    }
    ///Update an account
    pub fn update_account(&self, account_id: &str) -> request::UpdateAccountRequest {
        request::UpdateAccountRequest {
            client: &self,
            account_id: account_id.to_owned(),
            username: None,
            email: None,
            preferred_locale: None,
            cc_emails: None,
            first_name: None,
            last_name: None,
            company: None,
            vat_number: None,
            tax_exempt: None,
            exemption_certificate: None,
            parent_account_code: None,
            parent_account_id: None,
            bill_to: None,
            transaction_type: None,
            dunning_campaign_id: None,
            invoice_template_id: None,
            address: None,
            billing_info: None,
            custom_fields: None,
        }
    }
    /**Deactivate an account

Deactivating an account permanently deletes its billing information and cancels any active subscriptions (canceled subscriptions will remain active until the end of the current billing cycle before expiring). We recommend closing accounts only when all business is concluded with a customer.*/
    pub fn deactivate_account(
        &self,
        account_id: &str,
    ) -> request::DeactivateAccountRequest {
        request::DeactivateAccountRequest {
            client: &self,
            account_id: account_id.to_owned(),
        }
    }
    ///Fetch an account's acquisition data
    pub fn get_account_acquisition(
        &self,
        account_id: &str,
    ) -> request::GetAccountAcquisitionRequest {
        request::GetAccountAcquisitionRequest {
            client: &self,
            account_id: account_id.to_owned(),
        }
    }
    ///Update an account's acquisition data
    pub fn update_account_acquisition(
        &self,
        account_id: &str,
    ) -> request::UpdateAccountAcquisitionRequest {
        request::UpdateAccountAcquisitionRequest {
            client: &self,
            account_id: account_id.to_owned(),
            cost: None,
            channel: None,
            subchannel: None,
            campaign: None,
        }
    }
    ///Remove an account's acquisition data
    pub fn remove_account_acquisition(
        &self,
        account_id: &str,
    ) -> request::RemoveAccountAcquisitionRequest {
        request::RemoveAccountAcquisitionRequest {
            client: &self,
            account_id: account_id.to_owned(),
        }
    }
    /**Reactivate an inactive account

Reactivating an account will restore its history but the customer will need to provide new billing information to continue billing.*/
    pub fn reactivate_account(
        &self,
        account_id: &str,
    ) -> request::ReactivateAccountRequest {
        request::ReactivateAccountRequest {
            client: &self,
            account_id: account_id.to_owned(),
        }
    }
    ///Fetch an account's balance and past due status
    pub fn get_account_balance(
        &self,
        account_id: &str,
    ) -> request::GetAccountBalanceRequest {
        request::GetAccountBalanceRequest {
            client: &self,
            account_id: account_id.to_owned(),
        }
    }
    ///Fetch an account's billing information
    pub fn get_billing_info(&self, account_id: &str) -> request::GetBillingInfoRequest {
        request::GetBillingInfoRequest {
            client: &self,
            account_id: account_id.to_owned(),
        }
    }
    /**Set an account's billing information

If you're using Recurly.js to securely submit data from webforms without sending it through your server,
you can associate the billing information with an account by passing in the `token_id`. The only other
fields permitted with `token_id` are `primary_payment_method` and/or `backup_payment_method`.

For credit card payments you'll need the following required fields:

- first_name
- last_name
- number
- month
- year

For external (not Recurly.js) tokenized payments you'll need the following required fields:

- first_name
- last_name
- gateway_token
- gateway_code
*/
    pub fn update_billing_info(
        &self,
        account_id: &str,
    ) -> request::UpdateBillingInfoRequest {
        request::UpdateBillingInfoRequest {
            client: &self,
            account_id: account_id.to_owned(),
            token_id: None,
            first_name: None,
            last_name: None,
            company: None,
            address: None,
            number: None,
            month: None,
            year: None,
            cvv: None,
            vat_number: None,
            ip_address: None,
            gateway_token: None,
            gateway_code: None,
            amazon_billing_agreement_id: None,
            paypal_billing_agreement_id: None,
            fraud_session_id: None,
            transaction_type: None,
            three_d_secure_action_result_token_id: None,
            iban: None,
            name_on_account: None,
            account_number: None,
            routing_number: None,
            sort_code: None,
            type_: None,
            account_type: None,
            tax_identifier: None,
            tax_identifier_type: None,
            primary_payment_method: None,
            backup_payment_method: None,
            external_hpp_type: None,
            online_banking_payment_type: None,
            card_type: None,
        }
    }
    /**Remove an account's billing information

You may remove any stored billing information for an account. If the account has a subscription, the renewal will go into dunning unless the billing information is updated before the renewal occurs.*/
    pub fn remove_billing_info(
        &self,
        account_id: &str,
    ) -> request::RemoveBillingInfoRequest {
        request::RemoveBillingInfoRequest {
            client: &self,
            account_id: account_id.to_owned(),
        }
    }
    ///Verify an account's credit card billing information
    pub fn verify_billing_info(
        &self,
        account_id: &str,
    ) -> request::VerifyBillingInfoRequest {
        request::VerifyBillingInfoRequest {
            client: &self,
            account_id: account_id.to_owned(),
            gateway_code: None,
        }
    }
    ///Verify an account's credit card billing cvv
    pub fn verify_billing_info_cvv(
        &self,
        account_id: &str,
    ) -> request::VerifyBillingInfoCvvRequest {
        request::VerifyBillingInfoCvvRequest {
            client: &self,
            account_id: account_id.to_owned(),
            verification_value: None,
        }
    }
    /**Get the list of billing information associated with an account

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_billing_infos(
        &self,
        account_id: &str,
    ) -> request::ListBillingInfosRequest {
        request::ListBillingInfosRequest {
            client: &self,
            account_id: account_id.to_owned(),
            ids: None,
            sort: None,
            begin_time: None,
            end_time: None,
        }
    }
    /**Add new billing information on an account

If you're using Recurly.js to securely submit data from webforms without sending it through your server,
you can associate the billing information with an account by passing in the `token_id`. The only other
fields permitted with `token_id` are `primary_payment_method` and/or `backup_payment_method`.

For credit card payments you'll need the following required fields:

- first_name
- last_name
- number
- month
- year

For external (not Recurly.js) tokenized payments you'll need the following required fields:

- first_name
- last_name
- gateway_token
- gateway_code
*/
    pub fn create_billing_info(
        &self,
        account_id: &str,
    ) -> request::CreateBillingInfoRequest {
        request::CreateBillingInfoRequest {
            client: &self,
            account_id: account_id.to_owned(),
            token_id: None,
            first_name: None,
            last_name: None,
            company: None,
            address: None,
            number: None,
            month: None,
            year: None,
            cvv: None,
            vat_number: None,
            ip_address: None,
            gateway_token: None,
            gateway_code: None,
            amazon_billing_agreement_id: None,
            paypal_billing_agreement_id: None,
            fraud_session_id: None,
            transaction_type: None,
            three_d_secure_action_result_token_id: None,
            iban: None,
            name_on_account: None,
            account_number: None,
            routing_number: None,
            sort_code: None,
            type_: None,
            account_type: None,
            tax_identifier: None,
            tax_identifier_type: None,
            primary_payment_method: None,
            backup_payment_method: None,
            external_hpp_type: None,
            online_banking_payment_type: None,
            card_type: None,
        }
    }
    ///Fetch a billing info
    pub fn get_a_billing_info(
        &self,
        account_id: &str,
        billing_info_id: &str,
    ) -> request::GetABillingInfoRequest {
        request::GetABillingInfoRequest {
            client: &self,
            account_id: account_id.to_owned(),
            billing_info_id: billing_info_id.to_owned(),
        }
    }
    /**Update an account's billing information

If you're using Recurly.js to securely submit data from webforms without sending it through your server,
you can associate the billing information with an account by passing in the `token_id`. The only other
fields permitted with `token_id` are `primary_payment_method` and/or `backup_payment_method`.

For credit card payments you'll need the following required fields:

- first_name
- last_name
- number
- month
- year

For external (not Recurly.js) tokenized payments you'll need the following required fields:

- first_name
- last_name
- gateway_token
- gateway_code
*/
    pub fn update_a_billing_info(
        &self,
        account_id: &str,
        billing_info_id: &str,
    ) -> request::UpdateABillingInfoRequest {
        request::UpdateABillingInfoRequest {
            client: &self,
            account_id: account_id.to_owned(),
            billing_info_id: billing_info_id.to_owned(),
            token_id: None,
            first_name: None,
            last_name: None,
            company: None,
            address: None,
            number: None,
            month: None,
            year: None,
            cvv: None,
            vat_number: None,
            ip_address: None,
            gateway_token: None,
            gateway_code: None,
            amazon_billing_agreement_id: None,
            paypal_billing_agreement_id: None,
            fraud_session_id: None,
            transaction_type: None,
            three_d_secure_action_result_token_id: None,
            iban: None,
            name_on_account: None,
            account_number: None,
            routing_number: None,
            sort_code: None,
            type_: None,
            account_type: None,
            tax_identifier: None,
            tax_identifier_type: None,
            primary_payment_method: None,
            backup_payment_method: None,
            external_hpp_type: None,
            online_banking_payment_type: None,
            card_type: None,
        }
    }
    /**Remove an account's billing information

You may remove any stored billing information for an account. If the account has a subscription, the renewal will go into dunning unless the billing information is updated before the renewal occurs.*/
    pub fn remove_a_billing_info(
        &self,
        account_id: &str,
        billing_info_id: &str,
    ) -> request::RemoveABillingInfoRequest {
        request::RemoveABillingInfoRequest {
            client: &self,
            account_id: account_id.to_owned(),
            billing_info_id: billing_info_id.to_owned(),
        }
    }
    /**List the coupon redemptions for an account

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_account_coupon_redemptions(
        &self,
        account_id: &str,
    ) -> request::ListAccountCouponRedemptionsRequest {
        request::ListAccountCouponRedemptionsRequest {
            client: &self,
            account_id: account_id.to_owned(),
            ids: None,
            sort: None,
            begin_time: None,
            end_time: None,
            state: None,
        }
    }
    /**List the coupon redemptions that are active on an account

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_active_coupon_redemptions(
        &self,
        account_id: &str,
    ) -> request::ListActiveCouponRedemptionsRequest {
        request::ListActiveCouponRedemptionsRequest {
            client: &self,
            account_id: account_id.to_owned(),
        }
    }
    ///Generate an active coupon redemption on an account or subscription
    pub fn create_coupon_redemption(
        &self,
        account_id: &str,
        coupon_id: &str,
    ) -> request::CreateCouponRedemptionRequest {
        request::CreateCouponRedemptionRequest {
            client: &self,
            account_id: account_id.to_owned(),
            coupon_id: coupon_id.to_owned(),
            currency: None,
            subscription_id: None,
        }
    }
    ///Delete the active coupon redemption from an account
    pub fn remove_coupon_redemption(
        &self,
        account_id: &str,
    ) -> request::RemoveCouponRedemptionRequest {
        request::RemoveCouponRedemptionRequest {
            client: &self,
            account_id: account_id.to_owned(),
        }
    }
    /**List an account's credit payments

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_account_credit_payments(
        &self,
        account_id: &str,
    ) -> request::ListAccountCreditPaymentsRequest {
        request::ListAccountCreditPaymentsRequest {
            client: &self,
            account_id: account_id.to_owned(),
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
        }
    }
    /**List an account's invoices

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_account_invoices(
        &self,
        account_id: &str,
    ) -> request::ListAccountInvoicesRequest {
        request::ListAccountInvoicesRequest {
            client: &self,
            account_id: account_id.to_owned(),
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            type_: None,
        }
    }
    ///Create an invoice for pending line items
    pub fn create_invoice(
        &self,
        account_id: &str,
        currency: &str,
    ) -> request::CreateInvoiceRequest {
        request::CreateInvoiceRequest {
            client: &self,
            account_id: account_id.to_owned(),
            currency: currency.to_owned(),
            collection_method: None,
            charge_customer_notes: None,
            credit_customer_notes: None,
            net_terms: None,
            po_number: None,
            terms_and_conditions: None,
            vat_reverse_charge_notes: None,
        }
    }
    ///Preview new invoice for pending line items
    pub fn preview_invoice(
        &self,
        account_id: &str,
        currency: &str,
    ) -> request::PreviewInvoiceRequest {
        request::PreviewInvoiceRequest {
            client: &self,
            account_id: account_id.to_owned(),
            currency: currency.to_owned(),
            collection_method: None,
            charge_customer_notes: None,
            credit_customer_notes: None,
            net_terms: None,
            po_number: None,
            terms_and_conditions: None,
            vat_reverse_charge_notes: None,
        }
    }
    /**List an account's line items

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_account_line_items(
        &self,
        account_id: &str,
    ) -> request::ListAccountLineItemsRequest {
        request::ListAccountLineItemsRequest {
            client: &self,
            account_id: account_id.to_owned(),
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            original: None,
            state: None,
            type_: None,
        }
    }
    /**Create a new line item for the account

When using the Credit Invoices feature, utilize the purchases endpoint in order to immediately post credit to a credit invoice.*/
    pub fn create_line_item(
        &self,
        args: request::CreateLineItemRequired,
    ) -> request::CreateLineItemRequest {
        request::CreateLineItemRequest {
            client: &self,
            account_id: args.account_id.to_owned(),
            currency: args.currency.to_owned(),
            unit_amount: args.unit_amount,
            tax_inclusive: None,
            quantity: None,
            description: None,
            item_code: None,
            item_id: None,
            revenue_schedule_type: None,
            type_: args.type_.to_owned(),
            credit_reason_code: None,
            accounting_code: None,
            tax_exempt: None,
            avalara_transaction_type: None,
            avalara_service_type: None,
            tax_code: None,
            product_code: None,
            origin: None,
            start_date: None,
            end_date: None,
        }
    }
    /**List an account's notes

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_account_notes(
        &self,
        account_id: &str,
    ) -> request::ListAccountNotesRequest {
        request::ListAccountNotesRequest {
            client: &self,
            account_id: account_id.to_owned(),
            ids: None,
        }
    }
    ///Fetch an account note
    pub fn get_account_note(
        &self,
        account_id: &str,
        account_note_id: &str,
    ) -> request::GetAccountNoteRequest {
        request::GetAccountNoteRequest {
            client: &self,
            account_id: account_id.to_owned(),
            account_note_id: account_note_id.to_owned(),
        }
    }
    /**Fetch a list of an account's shipping addresses

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_shipping_addresses(
        &self,
        account_id: &str,
    ) -> request::ListShippingAddressesRequest {
        request::ListShippingAddressesRequest {
            client: &self,
            account_id: account_id.to_owned(),
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
        }
    }
    ///Create a new shipping address for the account
    pub fn create_shipping_address(
        &self,
        args: request::CreateShippingAddressRequired,
    ) -> request::CreateShippingAddressRequest {
        request::CreateShippingAddressRequest {
            client: &self,
            account_id: args.account_id.to_owned(),
            nickname: None,
            first_name: args.first_name.to_owned(),
            last_name: args.last_name.to_owned(),
            company: None,
            email: None,
            vat_number: None,
            phone: None,
            street1: args.street1.to_owned(),
            street2: None,
            city: args.city.to_owned(),
            region: None,
            postal_code: args.postal_code.to_owned(),
            country: args.country.to_owned(),
        }
    }
    ///Fetch an account's shipping address
    pub fn get_shipping_address(
        &self,
        account_id: &str,
        shipping_address_id: &str,
    ) -> request::GetShippingAddressRequest {
        request::GetShippingAddressRequest {
            client: &self,
            account_id: account_id.to_owned(),
            shipping_address_id: shipping_address_id.to_owned(),
        }
    }
    ///Update an account's shipping address
    pub fn update_shipping_address(
        &self,
        account_id: &str,
        shipping_address_id: &str,
    ) -> request::UpdateShippingAddressRequest {
        request::UpdateShippingAddressRequest {
            client: &self,
            account_id: account_id.to_owned(),
            shipping_address_id: shipping_address_id.to_owned(),
            id: None,
            nickname: None,
            first_name: None,
            last_name: None,
            company: None,
            email: None,
            vat_number: None,
            phone: None,
            street1: None,
            street2: None,
            city: None,
            region: None,
            postal_code: None,
            country: None,
        }
    }
    ///Remove an account's shipping address
    pub fn remove_shipping_address(
        &self,
        account_id: &str,
        shipping_address_id: &str,
    ) -> request::RemoveShippingAddressRequest {
        request::RemoveShippingAddressRequest {
            client: &self,
            account_id: account_id.to_owned(),
            shipping_address_id: shipping_address_id.to_owned(),
        }
    }
    /**List an account's subscriptions

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_account_subscriptions(
        &self,
        account_id: &str,
    ) -> request::ListAccountSubscriptionsRequest {
        request::ListAccountSubscriptionsRequest {
            client: &self,
            account_id: account_id.to_owned(),
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            state: None,
        }
    }
    /**List an account's transactions

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_account_transactions(
        &self,
        account_id: &str,
    ) -> request::ListAccountTransactionsRequest {
        request::ListAccountTransactionsRequest {
            client: &self,
            account_id: account_id.to_owned(),
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            type_: None,
            success: None,
        }
    }
    /**List an account's child accounts

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_child_accounts(
        &self,
        account_id: &str,
    ) -> request::ListChildAccountsRequest {
        request::ListChildAccountsRequest {
            client: &self,
            account_id: account_id.to_owned(),
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            email: None,
            subscriber: None,
            past_due: None,
        }
    }
    /**List a site's account acquisition data

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_account_acquisition(&self) -> request::ListAccountAcquisitionRequest {
        request::ListAccountAcquisitionRequest {
            client: &self,
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
        }
    }
    /**List a site's coupons

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_coupons(&self) -> request::ListCouponsRequest {
        request::ListCouponsRequest {
            client: &self,
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
        }
    }
    ///Create a new coupon
    pub fn create_coupon(
        &self,
        args: request::CreateCouponRequired,
    ) -> request::CreateCouponRequest {
        request::CreateCouponRequest {
            client: &self,
            name: args.name.to_owned(),
            max_redemptions: args.max_redemptions,
            max_redemptions_per_account: args.max_redemptions_per_account,
            hosted_description: args.hosted_description.to_owned(),
            invoice_description: args.invoice_description.to_owned(),
            redeem_by_date: args.redeem_by_date.to_owned(),
            code: args.code.to_owned(),
            discount_type: args.discount_type.to_owned(),
            discount_percent: args.discount_percent,
            free_trial_unit: args.free_trial_unit.to_owned(),
            free_trial_amount: args.free_trial_amount,
            currencies: args.currencies,
            applies_to_non_plan_charges: args.applies_to_non_plan_charges,
            applies_to_all_plans: args.applies_to_all_plans,
            applies_to_all_items: args.applies_to_all_items,
            plan_codes: args.plan_codes.iter().map(|&x| x.to_owned()).collect(),
            item_codes: args.item_codes.iter().map(|&x| x.to_owned()).collect(),
            duration: args.duration.to_owned(),
            temporal_amount: args.temporal_amount,
            temporal_unit: args.temporal_unit.to_owned(),
            coupon_type: args.coupon_type.to_owned(),
            unique_code_template: args.unique_code_template.to_owned(),
            redemption_resource: args.redemption_resource.to_owned(),
        }
    }
    ///Fetch a coupon
    pub fn get_coupon(&self, coupon_id: &str) -> request::GetCouponRequest {
        request::GetCouponRequest {
            client: &self,
            coupon_id: coupon_id.to_owned(),
        }
    }
    ///Update an active coupon
    pub fn update_coupon(&self, coupon_id: &str) -> request::UpdateCouponRequest {
        request::UpdateCouponRequest {
            client: &self,
            coupon_id: coupon_id.to_owned(),
            name: None,
            max_redemptions: None,
            max_redemptions_per_account: None,
            hosted_description: None,
            invoice_description: None,
            redeem_by_date: None,
        }
    }
    /**Expire a coupon

Mark an existing Coupon as expired*/
    pub fn deactivate_coupon(
        &self,
        coupon_id: &str,
    ) -> request::DeactivateCouponRequest {
        request::DeactivateCouponRequest {
            client: &self,
            coupon_id: coupon_id.to_owned(),
        }
    }
    ///Generate unique coupon codes
    pub fn generate_unique_coupon_codes(
        &self,
        coupon_id: &str,
    ) -> request::GenerateUniqueCouponCodesRequest {
        request::GenerateUniqueCouponCodesRequest {
            client: &self,
            coupon_id: coupon_id.to_owned(),
            number_of_unique_codes: None,
        }
    }
    /**Restore an inactive coupon

Make an expired coupon redeemable again. You can change editable fields in this call.*/
    pub fn restore_coupon(&self, coupon_id: &str) -> request::RestoreCouponRequest {
        request::RestoreCouponRequest {
            client: &self,
            coupon_id: coupon_id.to_owned(),
            name: None,
            max_redemptions: None,
            max_redemptions_per_account: None,
            hosted_description: None,
            invoice_description: None,
            redeem_by_date: None,
        }
    }
    /**List unique coupon codes associated with a bulk coupon

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_unique_coupon_codes(
        &self,
        coupon_id: &str,
    ) -> request::ListUniqueCouponCodesRequest {
        request::ListUniqueCouponCodesRequest {
            client: &self,
            coupon_id: coupon_id.to_owned(),
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
        }
    }
    /**List a site's credit payments

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_credit_payments(&self) -> request::ListCreditPaymentsRequest {
        request::ListCreditPaymentsRequest {
            client: &self,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
        }
    }
    ///Fetch a credit payment
    pub fn get_credit_payment(
        &self,
        credit_payment_id: &str,
    ) -> request::GetCreditPaymentRequest {
        request::GetCreditPaymentRequest {
            client: &self,
            credit_payment_id: credit_payment_id.to_owned(),
        }
    }
    /**List a site's custom field definitions

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_custom_field_definitions(
        &self,
    ) -> request::ListCustomFieldDefinitionsRequest {
        request::ListCustomFieldDefinitionsRequest {
            client: &self,
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            related_type: None,
        }
    }
    ///Fetch an custom field definition
    pub fn get_custom_field_definition(
        &self,
        custom_field_definition_id: &str,
    ) -> request::GetCustomFieldDefinitionRequest {
        request::GetCustomFieldDefinitionRequest {
            client: &self,
            custom_field_definition_id: custom_field_definition_id.to_owned(),
        }
    }
    /**List an invoice template's associated accounts

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_invoice_template_accounts(
        &self,
        invoice_template_id: &str,
    ) -> request::ListInvoiceTemplateAccountsRequest {
        request::ListInvoiceTemplateAccountsRequest {
            client: &self,
            invoice_template_id: invoice_template_id.to_owned(),
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            email: None,
            subscriber: None,
            past_due: None,
        }
    }
    /**List a site's items

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_items(&self) -> request::ListItemsRequest {
        request::ListItemsRequest {
            client: &self,
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            state: None,
        }
    }
    ///Create a new item
    pub fn create_item(&self, code: &str, name: &str) -> request::CreateItemRequest {
        request::CreateItemRequest {
            client: &self,
            code: code.to_owned(),
            name: name.to_owned(),
            description: None,
            external_sku: None,
            accounting_code: None,
            revenue_schedule_type: None,
            avalara_transaction_type: None,
            avalara_service_type: None,
            tax_code: None,
            tax_exempt: None,
            custom_fields: None,
            currencies: None,
        }
    }
    ///Fetch an item
    pub fn get_item(&self, item_id: &str) -> request::GetItemRequest {
        request::GetItemRequest {
            client: &self,
            item_id: item_id.to_owned(),
        }
    }
    ///Update an active item
    pub fn update_item(&self, item_id: &str) -> request::UpdateItemRequest {
        request::UpdateItemRequest {
            client: &self,
            item_id: item_id.to_owned(),
            code: None,
            name: None,
            description: None,
            external_sku: None,
            accounting_code: None,
            revenue_schedule_type: None,
            avalara_transaction_type: None,
            avalara_service_type: None,
            tax_code: None,
            tax_exempt: None,
            custom_fields: None,
            currencies: None,
        }
    }
    /**Deactivate an item

Deactivating an item makes it unavailable for new purchases. It will not affect existing line items.*/
    pub fn deactivate_item(&self, item_id: &str) -> request::DeactivateItemRequest {
        request::DeactivateItemRequest {
            client: &self,
            item_id: item_id.to_owned(),
        }
    }
    /**Reactivate an inactive item

Reactivating an item makes it available for new purchases. It will not affect existing line items.*/
    pub fn reactivate_item(&self, item_id: &str) -> request::ReactivateItemRequest {
        request::ReactivateItemRequest {
            client: &self,
            item_id: item_id.to_owned(),
        }
    }
    /**List a site's measured units

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_measured_unit(&self) -> request::ListMeasuredUnitRequest {
        request::ListMeasuredUnitRequest {
            client: &self,
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            state: None,
        }
    }
    ///Create a new measured unit
    pub fn create_measured_unit(
        &self,
        name: &str,
        display_name: &str,
    ) -> request::CreateMeasuredUnitRequest {
        request::CreateMeasuredUnitRequest {
            client: &self,
            name: name.to_owned(),
            display_name: display_name.to_owned(),
            description: None,
        }
    }
    ///Fetch a measured unit
    pub fn get_measured_unit(
        &self,
        measured_unit_id: &str,
    ) -> request::GetMeasuredUnitRequest {
        request::GetMeasuredUnitRequest {
            client: &self,
            measured_unit_id: measured_unit_id.to_owned(),
        }
    }
    ///Update a measured unit
    pub fn update_measured_unit(
        &self,
        measured_unit_id: &str,
    ) -> request::UpdateMeasuredUnitRequest {
        request::UpdateMeasuredUnitRequest {
            client: &self,
            measured_unit_id: measured_unit_id.to_owned(),
            name: None,
            display_name: None,
            description: None,
        }
    }
    /**Remove a measured unit

A mesured unit cannot be deleted if it is used by an active plan.*/
    pub fn remove_measured_unit(
        &self,
        measured_unit_id: &str,
    ) -> request::RemoveMeasuredUnitRequest {
        request::RemoveMeasuredUnitRequest {
            client: &self,
            measured_unit_id: measured_unit_id.to_owned(),
        }
    }
    /**List a site's external products

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_external_products(&self) -> request::ListExternalProductsRequest {
        request::ListExternalProductsRequest {
            client: &self,
            sort: None,
        }
    }
    ///Fetch an external product
    pub fn get_external_product(
        &self,
        external_product_id: &str,
    ) -> request::GetExternalProductRequest {
        request::GetExternalProductRequest {
            client: &self,
            external_product_id: external_product_id.to_owned(),
        }
    }
    /**List a site's external subscriptions

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_external_subscriptions(
        &self,
    ) -> request::ListExternalSubscriptionsRequest {
        request::ListExternalSubscriptionsRequest {
            client: &self,
            sort: None,
        }
    }
    ///Fetch an external subscription
    pub fn get_external_subscription(
        &self,
        external_subscription_id: &str,
    ) -> request::GetExternalSubscriptionRequest {
        request::GetExternalSubscriptionRequest {
            client: &self,
            external_subscription_id: external_subscription_id.to_owned(),
        }
    }
    /**List a site's invoices

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_invoices(&self) -> request::ListInvoicesRequest {
        request::ListInvoicesRequest {
            client: &self,
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            type_: None,
        }
    }
    ///Fetch an invoice
    pub fn get_invoice(&self, invoice_id: &str) -> request::GetInvoiceRequest {
        request::GetInvoiceRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
        }
    }
    ///Update an invoice
    pub fn update_invoice(&self, invoice_id: &str) -> request::UpdateInvoiceRequest {
        request::UpdateInvoiceRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
            po_number: None,
            vat_reverse_charge_notes: None,
            terms_and_conditions: None,
            customer_notes: None,
            net_terms: None,
            address: None,
        }
    }
    ///Fetch an invoice as a PDF
    pub fn get_invoice_pdf(&self, invoice_id: &str) -> request::GetInvoicePdfRequest {
        request::GetInvoicePdfRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
        }
    }
    /**Apply available credit to a pending or past due charge invoice

Apply credit payment to the outstanding balance on an existing charge invoice from an account’s available balance from existing credit invoices.*/
    pub fn apply_credit_balance(
        &self,
        site_id: &str,
        invoice_id: &str,
    ) -> request::ApplyCreditBalanceRequest {
        request::ApplyCreditBalanceRequest {
            client: &self,
            site_id: site_id.to_owned(),
            invoice_id: invoice_id.to_owned(),
        }
    }
    /**Collect a pending or past due, automatic invoice

Force a collection attempt using the stored billing information. This will trigger a transaction outside of Recurly's normal retry logic.*/
    pub fn collect_invoice(&self, invoice_id: &str) -> request::CollectInvoiceRequest {
        request::CollectInvoiceRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
            three_d_secure_action_result_token_id: None,
            transaction_type: None,
            billing_info_id: None,
        }
    }
    /**Mark an open invoice as failed

Indicates that the invoice was not successfully paid for and that collection attempts should stop. This functionality is mostly used to halt the dunning procedures for an invoice.

Only invoices with the `pending`, `processing` or `past_due` states can be marked as failed.
*/
    pub fn mark_invoice_failed(
        &self,
        invoice_id: &str,
    ) -> request::MarkInvoiceFailedRequest {
        request::MarkInvoiceFailedRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
        }
    }
    /**Mark an open invoice as successful

Indicates that the invoice was successfully paid for and that automated collection attempts should stop - this functionality is typically used to indicate that payment was received via another method and that revenue should be recognized.

Only invoices with the `pending`, `processing`, `past_due` or `failed` states can be marked as paid.
*/
    pub fn mark_invoice_successful(
        &self,
        invoice_id: &str,
    ) -> request::MarkInvoiceSuccessfulRequest {
        request::MarkInvoiceSuccessfulRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
        }
    }
    ///Reopen a closed, manual invoice
    pub fn reopen_invoice(&self, invoice_id: &str) -> request::ReopenInvoiceRequest {
        request::ReopenInvoiceRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
        }
    }
    /**Void a credit invoice.

Invoice must be a credit invoice (`type=credit`) and cannot be closed (`state=closed`), processing (`state=processing`), or already voided.*/
    pub fn void_invoice(&self, invoice_id: &str) -> request::VoidInvoiceRequest {
        request::VoidInvoiceRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
        }
    }
    /**Record an external payment for a manual invoices.

This endpoint allows you to record an offline payment that was not captured through your gateway. It will throw an error for an auto-collecting invoice.*/
    pub fn record_external_transaction(
        &self,
        invoice_id: &str,
    ) -> request::RecordExternalTransactionRequest {
        request::RecordExternalTransactionRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
            payment_method: None,
            description: None,
            amount: None,
            collected_at: None,
        }
    }
    /**List an invoice's line items

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_invoice_line_items(
        &self,
        invoice_id: &str,
    ) -> request::ListInvoiceLineItemsRequest {
        request::ListInvoiceLineItemsRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            original: None,
            state: None,
            type_: None,
        }
    }
    /**List the coupon redemptions applied to an invoice

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_invoice_coupon_redemptions(
        &self,
        invoice_id: &str,
    ) -> request::ListInvoiceCouponRedemptionsRequest {
        request::ListInvoiceCouponRedemptionsRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
            ids: None,
            sort: None,
            begin_time: None,
            end_time: None,
        }
    }
    /**List an invoice's related credit or charge invoices

Related invoices provide a link between credit invoices and the charge invoices that they are refunding.
For a charge invoice the related invoices will be credit invoices.
For a credit invoice the related invoices will be charge invoices.

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.
*/
    pub fn list_related_invoices(
        &self,
        invoice_id: &str,
    ) -> request::ListRelatedInvoicesRequest {
        request::ListRelatedInvoicesRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
        }
    }
    /**Refund an invoice

There are two ways to do a refund:
* refund a specific amount which is divided across all the line items.
* refund quantities of line items.
If you want to refund the entire refundable amount on the invoice, the
simplest way is to do `type=amount` without specifiying an `amount`.
*/
    pub fn refund_invoice(
        &self,
        invoice_id: &str,
        type_: &str,
    ) -> request::RefundInvoiceRequest {
        request::RefundInvoiceRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
            type_: type_.to_owned(),
            amount: None,
            line_items: None,
            refund_method: None,
            credit_customer_notes: None,
            external_refund: None,
        }
    }
    /**List a site's line items

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_line_items(&self) -> request::ListLineItemsRequest {
        request::ListLineItemsRequest {
            client: &self,
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            original: None,
            state: None,
            type_: None,
        }
    }
    ///Fetch a line item
    pub fn get_line_item(&self, line_item_id: &str) -> request::GetLineItemRequest {
        request::GetLineItemRequest {
            client: &self,
            line_item_id: line_item_id.to_owned(),
        }
    }
    ///Delete an uninvoiced line item
    pub fn remove_line_item(
        &self,
        line_item_id: &str,
    ) -> request::RemoveLineItemRequest {
        request::RemoveLineItemRequest {
            client: &self,
            line_item_id: line_item_id.to_owned(),
        }
    }
    /**List a site's plans

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_plans(&self) -> request::ListPlansRequest {
        request::ListPlansRequest {
            client: &self,
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            state: None,
        }
    }
    ///Create a plan
    pub fn create_plan(
        &self,
        code: &str,
        name: &str,
        currencies: Vec<PlanPricing>,
    ) -> request::CreatePlanRequest {
        request::CreatePlanRequest {
            client: &self,
            code: code.to_owned(),
            name: name.to_owned(),
            description: None,
            accounting_code: None,
            interval_unit: None,
            interval_length: None,
            trial_unit: None,
            trial_length: None,
            trial_requires_billing_info: None,
            total_billing_cycles: None,
            auto_renew: None,
            pricing_model: None,
            ramp_intervals: None,
            custom_fields: None,
            revenue_schedule_type: None,
            setup_fee_revenue_schedule_type: None,
            setup_fee_accounting_code: None,
            avalara_transaction_type: None,
            avalara_service_type: None,
            tax_code: None,
            tax_exempt: None,
            currencies,
            hosted_pages: None,
            add_ons: None,
            allow_any_item_on_subscriptions: None,
            dunning_campaign_id: None,
        }
    }
    ///Fetch a plan
    pub fn get_plan(&self, plan_id: &str) -> request::GetPlanRequest {
        request::GetPlanRequest {
            client: &self,
            plan_id: plan_id.to_owned(),
        }
    }
    ///Update a plan
    pub fn update_plan(&self, plan_id: &str) -> request::UpdatePlanRequest {
        request::UpdatePlanRequest {
            client: &self,
            plan_id: plan_id.to_owned(),
            id: None,
            code: None,
            name: None,
            description: None,
            accounting_code: None,
            trial_unit: None,
            trial_length: None,
            trial_requires_billing_info: None,
            total_billing_cycles: None,
            auto_renew: None,
            ramp_intervals: None,
            custom_fields: None,
            revenue_schedule_type: None,
            setup_fee_revenue_schedule_type: None,
            setup_fee_accounting_code: None,
            avalara_transaction_type: None,
            avalara_service_type: None,
            tax_code: None,
            tax_exempt: None,
            currencies: None,
            hosted_pages: None,
            allow_any_item_on_subscriptions: None,
            dunning_campaign_id: None,
        }
    }
    ///Remove a plan
    pub fn remove_plan(&self, plan_id: &str) -> request::RemovePlanRequest {
        request::RemovePlanRequest {
            client: &self,
            plan_id: plan_id.to_owned(),
        }
    }
    /**List a plan's add-ons

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_plan_add_ons(&self, plan_id: &str) -> request::ListPlanAddOnsRequest {
        request::ListPlanAddOnsRequest {
            client: &self,
            plan_id: plan_id.to_owned(),
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            state: None,
        }
    }
    ///Create an add-on
    pub fn create_plan_add_on(
        &self,
        plan_id: &str,
        code: &str,
        name: &str,
    ) -> request::CreatePlanAddOnRequest {
        request::CreatePlanAddOnRequest {
            client: &self,
            plan_id: plan_id.to_owned(),
            item_code: None,
            item_id: None,
            code: code.to_owned(),
            name: name.to_owned(),
            add_on_type: None,
            usage_type: None,
            usage_calculation_type: None,
            usage_percentage: None,
            measured_unit_id: None,
            measured_unit_name: None,
            accounting_code: None,
            revenue_schedule_type: None,
            display_quantity: None,
            default_quantity: None,
            optional: None,
            avalara_transaction_type: None,
            avalara_service_type: None,
            tax_code: None,
            currencies: None,
            tier_type: None,
            usage_timeframe: None,
            tiers: None,
            percentage_tiers: None,
        }
    }
    ///Fetch a plan's add-on
    pub fn get_plan_add_on(
        &self,
        plan_id: &str,
        add_on_id: &str,
    ) -> request::GetPlanAddOnRequest {
        request::GetPlanAddOnRequest {
            client: &self,
            plan_id: plan_id.to_owned(),
            add_on_id: add_on_id.to_owned(),
        }
    }
    ///Update an add-on
    pub fn update_plan_add_on(
        &self,
        plan_id: &str,
        add_on_id: &str,
    ) -> request::UpdatePlanAddOnRequest {
        request::UpdatePlanAddOnRequest {
            client: &self,
            plan_id: plan_id.to_owned(),
            add_on_id: add_on_id.to_owned(),
            id: None,
            code: None,
            name: None,
            usage_percentage: None,
            usage_calculation_type: None,
            measured_unit_id: None,
            measured_unit_name: None,
            accounting_code: None,
            revenue_schedule_type: None,
            avalara_transaction_type: None,
            avalara_service_type: None,
            tax_code: None,
            display_quantity: None,
            default_quantity: None,
            optional: None,
            currencies: None,
            tiers: None,
            percentage_tiers: None,
        }
    }
    ///Remove an add-on
    pub fn remove_plan_add_on(
        &self,
        plan_id: &str,
        add_on_id: &str,
    ) -> request::RemovePlanAddOnRequest {
        request::RemovePlanAddOnRequest {
            client: &self,
            plan_id: plan_id.to_owned(),
            add_on_id: add_on_id.to_owned(),
        }
    }
    /**List a site's add-ons

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_add_ons(&self) -> request::ListAddOnsRequest {
        request::ListAddOnsRequest {
            client: &self,
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            state: None,
        }
    }
    ///Fetch an add-on
    pub fn get_add_on(&self, add_on_id: &str) -> request::GetAddOnRequest {
        request::GetAddOnRequest {
            client: &self,
            add_on_id: add_on_id.to_owned(),
        }
    }
    /**List a site's shipping methods

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_shipping_methods(&self) -> request::ListShippingMethodsRequest {
        request::ListShippingMethodsRequest {
            client: &self,
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
        }
    }
    ///Create a new shipping method
    pub fn create_shipping_method(
        &self,
        code: &str,
        name: &str,
    ) -> request::CreateShippingMethodRequest {
        request::CreateShippingMethodRequest {
            client: &self,
            code: code.to_owned(),
            name: name.to_owned(),
            accounting_code: None,
            tax_code: None,
        }
    }
    ///Fetch a shipping method
    pub fn get_shipping_method(
        &self,
        shipping_method_id: &str,
    ) -> request::GetShippingMethodRequest {
        request::GetShippingMethodRequest {
            client: &self,
            shipping_method_id: shipping_method_id.to_owned(),
        }
    }
    ///Update an active Shipping Method
    pub fn update_shipping_method(
        &self,
        shipping_method_id: &str,
    ) -> request::UpdateShippingMethodRequest {
        request::UpdateShippingMethodRequest {
            client: &self,
            shipping_method_id: shipping_method_id.to_owned(),
            code: None,
            name: None,
            accounting_code: None,
            tax_code: None,
        }
    }
    /**Deactivate a shipping method

Deactivating a shipping method makes it unavailable for new subscriptions or purchases. It will not affect existing subscriptions.*/
    pub fn deactivate_shipping_method(
        &self,
        shipping_method_id: &str,
    ) -> request::DeactivateShippingMethodRequest {
        request::DeactivateShippingMethodRequest {
            client: &self,
            shipping_method_id: shipping_method_id.to_owned(),
        }
    }
    /**List a site's subscriptions

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_subscriptions(&self) -> request::ListSubscriptionsRequest {
        request::ListSubscriptionsRequest {
            client: &self,
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            state: None,
        }
    }
    ///Create a new subscription
    pub fn create_subscription(
        &self,
        plan_code: &str,
        account: AccountCreate,
        currency: &str,
    ) -> request::CreateSubscriptionRequest {
        request::CreateSubscriptionRequest {
            client: &self,
            plan_code: plan_code.to_owned(),
            plan_id: None,
            account,
            billing_info_id: None,
            shipping: None,
            collection_method: None,
            currency: currency.to_owned(),
            unit_amount: None,
            tax_inclusive: None,
            quantity: None,
            add_ons: None,
            coupon_codes: None,
            custom_fields: None,
            trial_ends_at: None,
            starts_at: None,
            next_bill_date: None,
            total_billing_cycles: None,
            renewal_billing_cycles: None,
            auto_renew: None,
            ramp_intervals: None,
            revenue_schedule_type: None,
            terms_and_conditions: None,
            customer_notes: None,
            credit_customer_notes: None,
            po_number: None,
            net_terms: None,
            transaction_type: None,
        }
    }
    ///Fetch a subscription
    pub fn get_subscription(
        &self,
        subscription_id: &str,
    ) -> request::GetSubscriptionRequest {
        request::GetSubscriptionRequest {
            client: &self,
            subscription_id: subscription_id.to_owned(),
        }
    }
    /**Update a subscription

This only lets you change the subscription settings that have no impact on the billed amount. Use the [Create Subscription Change](#operation/create_subscription_change) endpoint to make those changes.*/
    pub fn update_subscription(
        &self,
        subscription_id: &str,
    ) -> request::UpdateSubscriptionRequest {
        request::UpdateSubscriptionRequest {
            client: &self,
            subscription_id: subscription_id.to_owned(),
            collection_method: None,
            custom_fields: None,
            remaining_billing_cycles: None,
            renewal_billing_cycles: None,
            auto_renew: None,
            next_bill_date: None,
            revenue_schedule_type: None,
            terms_and_conditions: None,
            customer_notes: None,
            po_number: None,
            net_terms: None,
            gateway_code: None,
            tax_inclusive: None,
            shipping: None,
            billing_info_id: None,
        }
    }
    /**Terminate a subscription

Immediately expires the subscription.

If the subscription has a paid invoice you may choose to refund all, part or none of last invoice's amount.
*/
    pub fn terminate_subscription(
        &self,
        subscription_id: &str,
    ) -> request::TerminateSubscriptionRequest {
        request::TerminateSubscriptionRequest {
            client: &self,
            subscription_id: subscription_id.to_owned(),
            refund: None,
            charge: None,
        }
    }
    /**Cancel a subscription

A canceled subscription will continue through its current billing cycle. At the end of the current billing cycle the subscription will expire and the customer will no longer be billed. Canceled subscriptions can be reactivated until the end of the billing cycle. When a future subscription (`state=future`) is canceled it becomes failed `state=failed` and cannot be reactivated.*/
    pub fn cancel_subscription(
        &self,
        subscription_id: &str,
    ) -> request::CancelSubscriptionRequest {
        request::CancelSubscriptionRequest {
            client: &self,
            subscription_id: subscription_id.to_owned(),
            timeframe: None,
        }
    }
    /**Reactivate a canceled subscription

This will bring the subscription back to an active, renewing state on the customer's original billing cycle.

Expired or failed subscriptions cannot be reactivated; instead a new subscription plan will need to be applied to the account.
*/
    pub fn reactivate_subscription(
        &self,
        subscription_id: &str,
    ) -> request::ReactivateSubscriptionRequest {
        request::ReactivateSubscriptionRequest {
            client: &self,
            subscription_id: subscription_id.to_owned(),
        }
    }
    /**Pause subscription

This will put a subscription into the pause state at the next renewal. The body
of the request must contain the `remaining_pause_cycles` parameter. If the
subscription is currently paused and `remaining_pause_cycles` is 0, the subscription
will be resumed.

Expired, cancelled, or failed subscriptions cannot be paused.
*/
    pub fn pause_subscription(
        &self,
        subscription_id: &str,
        remaining_pause_cycles: i64,
    ) -> request::PauseSubscriptionRequest {
        request::PauseSubscriptionRequest {
            client: &self,
            subscription_id: subscription_id.to_owned(),
            remaining_pause_cycles,
        }
    }
    /**Resume subscription

This will immediately resume a paused subscription and move it into the
active state.

The subscription must be in the paused state. Active, expired, cancelled,
or failed subscriptions cannot be resumed.
*/
    pub fn resume_subscription(
        &self,
        subscription_id: &str,
    ) -> request::ResumeSubscriptionRequest {
        request::ResumeSubscriptionRequest {
            client: &self,
            subscription_id: subscription_id.to_owned(),
        }
    }
    /**Convert trial subscription

This will immediately convert a trial subscription to a fully active paid subscription, creating and collecting an invoice for auto-collecting subsriptions.  If the invoice cannot be collected, the subscription will remain in trial. The subscription must be in a trial. Active, paused, expired, cancelled, or failed subscriptions cannot be converted.*/
    pub fn convert_trial(&self, subscription_id: &str) -> request::ConvertTrialRequest {
        request::ConvertTrialRequest {
            client: &self,
            subscription_id: subscription_id.to_owned(),
        }
    }
    /**Fetch a preview of a subscription's renewal invoice(s)

The subscriptions's renewal invoice(s) will be returned if they exist; if they don't (for example, if the subscription is not set to renew), it will return an result with no invoices.*/
    pub fn get_preview_renewal(
        &self,
        subscription_id: &str,
    ) -> request::GetPreviewRenewalRequest {
        request::GetPreviewRenewalRequest {
            client: &self,
            subscription_id: subscription_id.to_owned(),
        }
    }
    ///Fetch a subscription's pending change
    pub fn get_subscription_change(
        &self,
        subscription_id: &str,
    ) -> request::GetSubscriptionChangeRequest {
        request::GetSubscriptionChangeRequest {
            client: &self,
            subscription_id: subscription_id.to_owned(),
        }
    }
    /**Create a new subscription change

Calling this will overwrite an existing, pending subscription change.

If a subscription has a pending change, and a change is submitted which matches
the subscription as it currently exists, the pending change will be deleted,
and you will receive a 204 No Content response.

If a subscription has no pending
change, and a change is submitted which matches the subscription as it currently
exists, a 422 Unprocessable Entity validation error will be returned.
*/
    pub fn create_subscription_change(
        &self,
        subscription_id: &str,
    ) -> request::CreateSubscriptionChangeRequest {
        request::CreateSubscriptionChangeRequest {
            client: &self,
            subscription_id: subscription_id.to_owned(),
            timeframe: None,
            plan_id: None,
            plan_code: None,
            unit_amount: None,
            tax_inclusive: None,
            quantity: None,
            shipping: None,
            coupon_codes: None,
            add_ons: None,
            collection_method: None,
            revenue_schedule_type: None,
            custom_fields: None,
            po_number: None,
            net_terms: None,
            transaction_type: None,
            billing_info: None,
            ramp_intervals: None,
        }
    }
    /**Delete the pending subscription change

Deleting the pending subscription change will cause the current subscription settings to be used on the next renewal.*/
    pub fn remove_subscription_change(
        &self,
        subscription_id: &str,
    ) -> request::RemoveSubscriptionChangeRequest {
        request::RemoveSubscriptionChangeRequest {
            client: &self,
            subscription_id: subscription_id.to_owned(),
        }
    }
    /**Preview a new subscription change

Calling this will not save the subscription change or overwrite an existing change.*/
    pub fn preview_subscription_change(
        &self,
        subscription_id: &str,
    ) -> request::PreviewSubscriptionChangeRequest {
        request::PreviewSubscriptionChangeRequest {
            client: &self,
            subscription_id: subscription_id.to_owned(),
            timeframe: None,
            plan_id: None,
            plan_code: None,
            unit_amount: None,
            tax_inclusive: None,
            quantity: None,
            shipping: None,
            coupon_codes: None,
            add_ons: None,
            collection_method: None,
            revenue_schedule_type: None,
            custom_fields: None,
            po_number: None,
            net_terms: None,
            transaction_type: None,
            billing_info: None,
            ramp_intervals: None,
        }
    }
    /**List a subscription's invoices

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_subscription_invoices(
        &self,
        subscription_id: &str,
    ) -> request::ListSubscriptionInvoicesRequest {
        request::ListSubscriptionInvoicesRequest {
            client: &self,
            subscription_id: subscription_id.to_owned(),
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            type_: None,
        }
    }
    /**List a subscription's line items

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_subscription_line_items(
        &self,
        subscription_id: &str,
    ) -> request::ListSubscriptionLineItemsRequest {
        request::ListSubscriptionLineItemsRequest {
            client: &self,
            subscription_id: subscription_id.to_owned(),
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            original: None,
            state: None,
            type_: None,
        }
    }
    /**List the coupon redemptions for a subscription

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_subscription_coupon_redemptions(
        &self,
        subscription_id: &str,
    ) -> request::ListSubscriptionCouponRedemptionsRequest {
        request::ListSubscriptionCouponRedemptionsRequest {
            client: &self,
            subscription_id: subscription_id.to_owned(),
            ids: None,
            sort: None,
            begin_time: None,
            end_time: None,
        }
    }
    ///List a subscription add-on's usage records
    pub fn list_usage(
        &self,
        subscription_id: &str,
        add_on_id: &str,
    ) -> request::ListUsageRequest {
        request::ListUsageRequest {
            client: &self,
            subscription_id: subscription_id.to_owned(),
            add_on_id: add_on_id.to_owned(),
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            billing_status: None,
        }
    }
    ///Log a usage record on this subscription add-on
    pub fn create_usage(
        &self,
        subscription_id: &str,
        add_on_id: &str,
    ) -> request::CreateUsageRequest {
        request::CreateUsageRequest {
            client: &self,
            subscription_id: subscription_id.to_owned(),
            add_on_id: add_on_id.to_owned(),
            merchant_tag: None,
            amount: None,
            recording_timestamp: None,
            usage_timestamp: None,
        }
    }
    ///Get a usage record
    pub fn get_usage(&self, usage_id: &str) -> request::GetUsageRequest {
        request::GetUsageRequest {
            client: &self,
            usage_id: usage_id.to_owned(),
        }
    }
    ///Update a usage record
    pub fn update_usage(&self, usage_id: &str) -> request::UpdateUsageRequest {
        request::UpdateUsageRequest {
            client: &self,
            usage_id: usage_id.to_owned(),
            merchant_tag: None,
            amount: None,
            recording_timestamp: None,
            usage_timestamp: None,
        }
    }
    ///Delete a usage record.
    pub fn remove_usage(&self, usage_id: &str) -> request::RemoveUsageRequest {
        request::RemoveUsageRequest {
            client: &self,
            usage_id: usage_id.to_owned(),
        }
    }
    /**List a site's transactions

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_transactions(&self) -> request::ListTransactionsRequest {
        request::ListTransactionsRequest {
            client: &self,
            ids: None,
            limit: None,
            order: None,
            sort: None,
            begin_time: None,
            end_time: None,
            type_: None,
            success: None,
        }
    }
    ///Fetch a transaction
    pub fn get_transaction(
        &self,
        transaction_id: &str,
    ) -> request::GetTransactionRequest {
        request::GetTransactionRequest {
            client: &self,
            transaction_id: transaction_id.to_owned(),
        }
    }
    ///Fetch a unique coupon code
    pub fn get_unique_coupon_code(
        &self,
        unique_coupon_code_id: &str,
    ) -> request::GetUniqueCouponCodeRequest {
        request::GetUniqueCouponCodeRequest {
            client: &self,
            unique_coupon_code_id: unique_coupon_code_id.to_owned(),
        }
    }
    /**Deactivate a unique coupon code

Expire a unique code, making that specific code no longer redeemable. The parent bulk coupon will not be affected.*/
    pub fn deactivate_unique_coupon_code(
        &self,
        unique_coupon_code_id: &str,
    ) -> request::DeactivateUniqueCouponCodeRequest {
        request::DeactivateUniqueCouponCodeRequest {
            client: &self,
            unique_coupon_code_id: unique_coupon_code_id.to_owned(),
        }
    }
    ///Restore a unique coupon code
    pub fn reactivate_unique_coupon_code(
        &self,
        unique_coupon_code_id: &str,
    ) -> request::ReactivateUniqueCouponCodeRequest {
        request::ReactivateUniqueCouponCodeRequest {
            client: &self,
            unique_coupon_code_id: unique_coupon_code_id.to_owned(),
        }
    }
    /**Create a new purchase

A purchase is a checkout containing at least one or more subscriptions or one-time charges (line items) and supports both coupon and gift card redemptions. All items purchased will be on one invoice and paid for with one transaction.*/
    pub fn create_purchase(
        &self,
        currency: &str,
        account: AccountPurchase,
    ) -> request::CreatePurchaseRequest {
        request::CreatePurchaseRequest {
            client: &self,
            currency: currency.to_owned(),
            account,
            billing_info_id: None,
            collection_method: None,
            po_number: None,
            net_terms: None,
            terms_and_conditions: None,
            customer_notes: None,
            vat_reverse_charge_notes: None,
            credit_customer_notes: None,
            gateway_code: None,
            shipping: None,
            line_items: None,
            subscriptions: None,
            coupon_codes: None,
            gift_card_redemption_code: None,
            transaction_type: None,
        }
    }
    /**Preview a new purchase

A purchase is a checkout containing at least one or more subscriptions or one-time charges (line items) and supports both coupon and gift card redemptions. All items purchased will be on one invoice and paid for with one transaction.*/
    pub fn preview_purchase(
        &self,
        currency: &str,
        account: AccountPurchase,
    ) -> request::PreviewPurchaseRequest {
        request::PreviewPurchaseRequest {
            client: &self,
            currency: currency.to_owned(),
            account,
            billing_info_id: None,
            collection_method: None,
            po_number: None,
            net_terms: None,
            terms_and_conditions: None,
            customer_notes: None,
            vat_reverse_charge_notes: None,
            credit_customer_notes: None,
            gateway_code: None,
            shipping: None,
            line_items: None,
            subscriptions: None,
            coupon_codes: None,
            gift_card_redemption_code: None,
            transaction_type: None,
        }
    }
    /**Create a pending purchase

A purchase is a hybrid checkout containing at least one or more subscriptions or one-time charges (adjustments) and supports both coupon and gift card redemptions. All items purchased will be on one invoice and paid for with one transaction. A purchase is only a request data type and is not persistent in Recurly and an invoice collection will be the returned type.

Use for **Adyen HPP** and **Online Banking** transaction requests.
This runs the validations but not the transactions.
The API request allows the inclusion of one of the following fields: **external_hpp_type** with `'adyen'` and **online_banking_payment_type** with `'ideal'` or `'sofort'` in the **billing_info** object.

For additional information regarding shipping fees, please see https://docs.recurly.com/docs/shipping*/
    pub fn create_pending_purchase(
        &self,
        currency: &str,
        account: AccountPurchase,
    ) -> request::CreatePendingPurchaseRequest {
        request::CreatePendingPurchaseRequest {
            client: &self,
            currency: currency.to_owned(),
            account,
            billing_info_id: None,
            collection_method: None,
            po_number: None,
            net_terms: None,
            terms_and_conditions: None,
            customer_notes: None,
            vat_reverse_charge_notes: None,
            credit_customer_notes: None,
            gateway_code: None,
            shipping: None,
            line_items: None,
            subscriptions: None,
            coupon_codes: None,
            gift_card_redemption_code: None,
            transaction_type: None,
        }
    }
    /**List the dates that have an available export to download.

Returns a list of dates for which export files are available for download.*/
    pub fn get_export_dates(&self) -> request::GetExportDatesRequest {
        request::GetExportDatesRequest {
            client: &self,
        }
    }
    /**List of the export files that are available to download.

Returns a list of presigned URLs to download export files for the given date, with their MD5 sums.*/
    pub fn get_export_files(&self, export_date: &str) -> request::GetExportFilesRequest {
        request::GetExportFilesRequest {
            client: &self,
            export_date: export_date.to_owned(),
        }
    }
    /**List the dunning campaigns for a site

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_dunning_campaigns(&self) -> request::ListDunningCampaignsRequest {
        request::ListDunningCampaignsRequest {
            client: &self,
            sort: None,
        }
    }
    ///Fetch a dunning campaign
    pub fn get_dunning_campaign(
        &self,
        dunning_campaign_id: &str,
    ) -> request::GetDunningCampaignRequest {
        request::GetDunningCampaignRequest {
            client: &self,
            dunning_campaign_id: dunning_campaign_id.to_owned(),
        }
    }
    ///Assign a dunning campaign to multiple plans
    pub fn put_dunning_campaign_bulk_update(
        &self,
        dunning_campaign_id: &str,
    ) -> request::PutDunningCampaignBulkUpdateRequest {
        request::PutDunningCampaignBulkUpdateRequest {
            client: &self,
            dunning_campaign_id: dunning_campaign_id.to_owned(),
            plan_codes: None,
            plan_ids: None,
        }
    }
    /**Show the invoice templates for a site

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_invoice_templates(&self) -> request::ListInvoiceTemplatesRequest {
        request::ListInvoiceTemplatesRequest {
            client: &self,
            sort: None,
        }
    }
    ///Fetch an invoice template
    pub fn get_invoice_template(
        &self,
        invoice_template_id: &str,
    ) -> request::GetInvoiceTemplateRequest {
        request::GetInvoiceTemplateRequest {
            client: &self,
            invoice_template_id: invoice_template_id.to_owned(),
        }
    }
    ///List entitlements granted to an account
    pub fn list_entitlements(
        &self,
        account_id: &str,
    ) -> request::ListEntitlementsRequest {
        request::ListEntitlementsRequest {
            client: &self,
            account_id: account_id.to_owned(),
            state: None,
        }
    }
    /**List an account's external subscriptions

See the [Pagination Guide](/developers/guides/pagination.html) to learn how to use pagination in the API and Client Libraries.*/
    pub fn list_account_external_subscriptions(
        &self,
        account_id: &str,
    ) -> request::ListAccountExternalSubscriptionsRequest {
        request::ListAccountExternalSubscriptionsRequest {
            client: &self,
            sort: None,
            account_id: account_id.to_owned(),
        }
    }
}
pub enum RecurlyAuthentication {
    ApiKey { api_key: String },
}
impl RecurlyAuthentication {
    pub fn from_env() -> Self {
        Self::ApiKey {
            api_key: std::env::var("RECURLY_API_KEY")
                .expect("Environment variable RECURLY_API_KEY is not set."),
        }
    }
}
