use crate::endpoints::orders::schema::address::{Address, PersonFullName, PhoneWithType};
use crate::endpoints::orders::schema::source::token::PaymentUsage;
use crate::endpoints::orders::schema::source::{
    PaymentInitiator, ShippingPreference, VaultStoreInValue,
};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PaypalSource {
    /// Customizes the payer experience during the approval process for payment with PayPal.
    #[builder(default, setter(strip_option, into))]
    pub experience_context: Option<PaypalExperienceContext>,

    /// The PayPal billing agreement ID.
    /// References an approved recurring payment for goods or services.
    #[builder(default, setter(strip_option, into))]
    pub billing_agreement_id: Option<String>,

    /// Provides additional details to process a payment using the PayPal wallet billing agreement or a vaulted payment method that has been stored or is intended to be stored.
    #[builder(default, setter(strip_option, into))]
    pub stored_credential: Option<PaypalStoredCredential>,

    /// The PayPal-generated ID for the payment_source stored within the Vault.
    #[builder(default, setter(strip_option, into))]
    pub vault_id: Option<String>,

    /// The email address of the PayPal account holder.
    #[builder(default, setter(strip_option, into))]
    pub email_address: Option<String>,

    /// The name of the PayPal account holder. Supports only the given_name and surname properties.
    #[builder(default, setter(strip_option, into))]
    pub name: Option<PersonFullName>,

    /// The phone number of the customer.
    /// Available only when you enable the Contact Telephone Number option in the Profile & Settings for the merchant's PayPal account.
    /// The phone.phone_number supports only national_number.
    #[builder(default, setter(strip_option, into))]
    pub phone: Option<PhoneWithType>,

    /// The birth date of the PayPal account holder in YYYY-MM-DD format.
    #[builder(default, setter(strip_option, into))]
    pub birth_date: Option<String>,

    /// The tax information of the PayPal account holder.
    /// Required only for Brazilian PayPal account holder's.
    /// Both tax_id and tax_id_type are required.
    #[builder(default, setter(strip_option, into))]
    pub tax_info: Option<PaypalTaxInfo>,

    /// The address of the PayPal account holder. Supports only the address_line_1, address_line_2, admin_area_1, admin_area_2, postal_code, and country_code properties.
    /// Also referred to as the billing address of the customer.
    #[builder(default, setter(strip_option, into))]
    pub address: Option<Address>,

    /// Additional attributes associated with the use of this wallet.
    #[builder(default, setter(strip_option, into))]
    pub attributes: Option<PaypalAttributes>,
}

#[derive(Debug, Serialize, Deserialize, Default, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::enum_variant_names)]
pub enum PaypalContactPreference {
    /// The merchant can opt out of showing buyer's contact information on PayPal checkout.
    #[default]
    NoContactInfo,
    /// The merchant allows buyer to add or update shipping contact information on the PayPal checkout.
    /// Please ensure to use this updated information returned in shipping.email_address and shipping.phone_number to contact your buyers.
    UpdateContactInfo,
    /// The buyer can only see but can not override merchant passed contact information (shipping.email_address and shipping.phone_number) on PayPal checkout.
    /// NOTE: If you don't pass the contact information, the behavior is the same as NO_CONTACT_INFO preference.
    RetainContactInfo,
}

#[derive(Debug, Serialize, Deserialize, Default, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum PaypalLandingPage {
    /// When the customer clicks PayPal Checkout, the customer is redirected to a page to log in to PayPal and approve the payment.
    Login,

    /// When the customer clicks PayPal Checkout, the customer is redirected to a page to enter credit or debit card and other relevant billing information required to complete the purchase.
    /// This option has previously been also called as 'BILLING'
    GuestCheckout,

    /// When the customer clicks PayPal Checkout,
    /// the customer is redirected to either a page to log in to PayPal and approve the payment
    /// or to a page to enter credit or debit card and other relevant billing information required to complete the purchase,
    /// depending on their previous interaction with PayPal.
    #[default]
    NoPreference,
}

#[derive(Debug, Serialize, Deserialize, Default, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum PaypalUserAction {
    /// After you redirect the customer to the PayPal payment page, a Continue button appears.
    /// Use this option when the final amount is not known when the checkout flow is initiated and you want to redirect the customer to the merchant page without processing the payment.
    #[default]
    Continue,

    /// After you redirect the customer to the PayPal payment page, a Pay Now button appears.
    /// Use this option when the final amount is known when the checkout is initiated and you want to process the payment immediately when the customer clicks Pay Now.
    PayNow,
}

#[derive(Debug, Serialize, Deserialize, Default, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum PaypalPaymentMethodPreference {
    /// Accepts any type of payment from the customer.
    #[default]
    Unrestricted,

    /// Accepts only immediate payment from the customer.
    /// For example, credit card, PayPal balance, or instant ACH.
    /// Ensures that at the time of capture, the payment does not have the pending status.
    ImmediatePaymentRequired,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PaypalOrderUpdateCallbackConfig {
    /// An array of callback events merchant can subscribe to for the corresponding callback url.
    #[builder(setter(into))]
    pub callback_events: Vec<String>,

    /// Merchant provided CallBack url.PayPal/Venmo will use this url to call the merchant back when the events occur .
    /// PayPal/Venmo expects a secured url usually in the https format.merchant can append the cart id or other params part of the url as query or path params.
    #[builder(setter(into))]
    pub callback_url: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PaypalExperienceContext {
    /// The label that overrides the business name in the PayPal account on the PayPal site. The pattern is defined by an external party and supports Unicode.
    #[builder(default, setter(strip_option, into))]
    pub brand_name: Option<String>,

    /// The location from which the shipping address is derived.
    #[builder(default, setter(into))]
    pub shipping_preference: ShippingPreference,

    /// The location from which the shipping address is derived.
    #[builder(default, setter(into))]
    pub contact_preference: PaypalContactPreference,

    /// The type of landing page to show on the PayPal site for customer checkout.
    #[builder(default, setter(into))]
    pub landing_page: PaypalLandingPage,

    /// Configures a Continue or Pay Now checkout flow.
    #[builder(default, setter(into))]
    pub user_action: PaypalUserAction,

    /// The merchant-preferred payment methods.
    #[builder(default, setter(into))]
    pub payment_method_preference: PaypalPaymentMethodPreference,

    /// The BCP 47-formatted locale of pages that the PayPal payment experience shows. PayPal supports a five-character code.
    /// For example, da-DK, he-IL, id-ID, ja-JP, no-NO, pt-BR, ru-RU, sv-SE, th-TH, zh-CN, zh-HK, or zh-TW.
    #[builder(default, setter(strip_option, into))]
    pub locale: Option<String>,

    /// The URL where the customer will be redirected upon approving a payment.
    #[builder(default, setter(strip_option, into))]
    pub return_url: Option<String>,

    /// The URL where the customer will be redirected upon cancelling the payment approval.
    #[builder(default, setter(strip_option, into))]
    pub cancel_url: Option<String>,

    /// Merchant provided Order Update callback configuration for PayPal Wallet.PayPal will call back merchant when the specified event occurs.
    /// we recommend merchants to pass both the shipping_options and shipping_address callback events.
    /// Not supported when shipping.type is specified or when 'application_context.shipping_preference' is set as 'NO_SHIPPING' or 'SET_PROVIDED_ADDRESS'.
    #[builder(default, setter(strip_option, into))]
    pub order_update_callback_config: Option<PaypalOrderUpdateCallbackConfig>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum PaypalCredentialPattern {
    /// On-demand instant payments – non-recurring, pre-paid, variable amount, variable frequency.
    Immediate,
    /// Pay after use, non-recurring post-paid, variable amount, irregular frequency.
    Deferred,
    /// Pay upfront fixed or variable amount on a fixed date before the goods/service is delivered.
    RecurringPrepaid,
    /// Pay on a fixed date based on usage or consumption after the goods/service is delivered.
    RecurringPostpaid,
    /// Charge payer when the set amount is reached or monthly billing cycle, whichever comes first, before the goods/service is delivered.
    ThresholdPrepaid,
    /// Charge payer when the set amount is reached or monthly billing cycle, whichever comes first, after the goods/service is delivered.
    ThresholdPostpaid,
    /// Subscription plan where the "amount due" and the "billing frequency" are fixed, and there is no defined duration with the payment due before the good/service is delivered.
    SubscriptionPrepaid,
    /// Subscription plan where the "amount due" and the "billing frequency" are fixed, and there is no defined duration with the payment due after the goods/services are delivered.
    SubscriptionPostpaid,
    /// Unscheduled card on file plan where the merchant can bill buyer upfront based on an agreed logic, but "amount due" and "frequency" can vary. Inclusive of automatic reload plans.
    UnscheduledPrepaid,
    /// Unscheduled card on file plan where the merchant can bill buyer based on an agreed logic, but "amount due" and "frequency" can vary. Inclusive of automatic reload plans.
    UnscheduledPostpaid,
    /// Merchant-managed installment plan when the "amount" to be paid and the "billing frequency" are fixed, but there is a defined number of payments with the payment due before the good/service is delivered.
    InstallmentPrepaid,
    /// Merchant-managed installment plan when the "amount" to be paid and the "billing frequency" are fixed, but there is a defined number of payments with the payment due after the goods/services are delivered.
    InstallmentPostpaid,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PaypalStoredCredential {
    /// The person or party who initiated or triggered the payment.
    #[builder(setter(into))]
    pub payment_initiator: PaymentInitiator,

    /// DEPRECATED. Expected business/pricing model for the billing agreement, Please use usage_pattern instead.
    #[builder(default, setter(strip_option, into))]
    pub charge_pattern: Option<PaypalCredentialPattern>,

    /// Expected business/pricing model for the billing agreement.
    #[builder(default, setter(strip_option, into))]
    pub usage_pattern: Option<PaypalCredentialPattern>,

    /// Indicates if this is a first or subsequent payment using a stored payment source (also referred to as stored credential or card on file).
    #[builder(default, setter(into))]
    pub usage: PaymentUsage,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum PaypalTaxIdType {
    /// The individual tax ID type, typically is 11 characters long.
    BrCpf,

    /// The business tax ID type, typically is 14 characters long.
    BrCnpj,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PaypalTaxInfo {
    /// The customer's tax ID value.
    #[builder(setter(into))]
    pub tax_id: String,

    /// The customer's tax ID type.
    #[builder(setter(into))]
    pub tax_id_type: PaypalTaxIdType,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PaypalAttributes {
    /// The details about a customer in PayPal's system of record.
    #[builder(default, setter(strip_option, into))]
    pub customer: Option<PaypalCustomer>,

    /// Attributes used to provide the instructions during vaulting of the PayPal Wallet.
    #[builder(default, setter(strip_option, into))]
    pub vault: Option<PaypalVault>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PaypalCustomer {
    /// The unique ID for a customer generated by PayPal.
    #[builder(default, setter(strip_option, into))]
    pub id: Option<String>,

    /// Email address of the customer as provided to the merchant or on file with the merchant.
    /// Email Address is required if you are processing the transaction using PayPal Guest Processing which is offered to select partners and merchants.
    #[builder(default, setter(strip_option, into))]
    pub email_address: Option<String>,

    /// The phone number of the customer as provided to the merchant or on file with the merchant.
    /// The phone.phone_number supports only national_number.
    #[builder(default, setter(strip_option, into))]
    pub phone: Option<PhoneWithType>,

    /// The full name of the customer as provided to the merchant or on file with the merchant.
    #[builder(default, setter(strip_option, into))]
    pub name: Option<PersonFullName>,

    /// Merchants and partners may already have a data-store where their customer information is persisted.
    /// Use merchant_customer_id to associate the PayPal-generated customer.id to your representation of a customer.
    #[builder(default, setter(strip_option, into))]
    pub merchant_customer_id: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PaypalVault {
    /// Defines how and when the payment source gets vaulted.
    #[builder(default, setter(strip_option, into))]
    pub store_in_vault: Option<VaultStoreInValue>,

    /// The description displayed to PayPal consumer on the approval flow for PayPal,
    /// as well as on the PayPal payment token management experience on PayPal.com.
    #[builder(default, setter(strip_option, into))]
    pub description: Option<String>,

    /// Expected business/pricing model for the billing agreement.
    #[builder(default, setter(strip_option, into))]
    pub usage_pattern: Option<String>,

    /// The usage type associated with the PayPal payment token.
    #[builder(setter(into))]
    pub usage_type: String,

    /// The customer type associated with the PayPal payment token. This is to indicate whether the customer acting on the merchant / platform is either a business or a consumer.
    #[builder(default, setter(strip_option, into))]
    pub customer_type: Option<String>,

    /// Create multiple payment tokens for the same payer, merchant/platform combination.
    /// Use this when the customer has not logged in at merchant/platform.
    /// The payment token thus generated, can then also be used to create the customer account at merchant/platform.
    /// Use this also when multiple payment tokens are required for the same payer, different customer at merchant/platform.
    /// This helps to identify customers distinctly even though they may share the same PayPal account.
    /// This only applies to PayPal payment source.
    #[builder(setter(strip_bool))]
    pub permit_multiple_payment_tokens: bool,
}
