#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Orders {
  pub result: Vec<Order>,
  pub pagination: Pagination,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
  pub id: String,
  pub order_number: u64,
  // TODO: parse this into some usable time format instead of pure string
  pub created_on: String,
  pub modified_on: String,
  pub channel: String,
  pub testmode: bool,
  pub customer_email: String,
  pub billing_address: Address,
  pub shipping_address: Option<Address>,
  // TODO: This can be an enum
  // Value may be: `PENDING`, `FULFILLED`, or `CANCELED`.
  pub fulfilment_status: String,
  pub line_items: Vec<LineItem>,
  pub internal_notes: Vec<InternalNote>,
  pub shipping_lines: Vec<ShippingLine>,
  pub discount_lines: Vec<DiscountLine>,
  pub form_submission: Vec<FormSubmission>,
  pub fulfillments: Vec<Fulfillment>,
  pub subtotal: MonetaryValue,
  pub shipping_total: MonetaryValue,
  pub discount_total: MonetaryValue,
  pub tax_total: MonetaryValue,
  pub refunded_total: MonetaryValue,
  pub grand_total: MonetaryValue,
  pub channel_name: String,
  pub external_order_reference: Option<String>,
  // TODO: make this a timestamp or something
  pub fulfilled_on: String,
  // TODO: make this an enum
  // Values may be `EXCLUSIVE` or `INCLUSIVE`.
  pub price_tax_interpretation: String
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Fulfillment {
  // ISO 8601 UTC date and time string; represents the moment the fulfillment was shipped.
  pub ship_date: String,
  // Name of the carrier handling the shipment.
  pub carrier_name: String,
  // Carrier's level of service for shipping.
  pub service: String,
  // Carrier's parcel tracking number.
  pub tracking_number: String,
  // URL provided by the carrier to track the shipment.
  pub tracking_url: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscountLine {
  pub description: String,
  pub name: String,
  pub amount: MonetaryValue,
  pub promo_code: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FormSubmission {
  pub label: String,
  pub value: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InternalNote {
  pub content: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShippingLine {
  pub method: String,
  pub amount: MonetaryValue,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
  pub has_next_page: bool,
  pub next_page_cursor: Option<String>,
  pub next_page_url: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Address {
  pub first_name: String,
  pub last_name: String,
  pub address_1: String,
  pub address_2: Option<String>,
  pub city: String,
  pub state: String,
  pub country_code: String,
  pub postal_code: String,
  pub phone: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LineItem {
  pub id: String,
  pub variant_id: Option<String>,
  pub sku: String,
  pub weight: f64,
  pub width: f64,
  pub length: f64,
  pub height: f64,
  pub product_id: Option<String>,
  pub product_name: Option<String>,
  pub quantity: u64,
  pub unit_price_paid: MonetaryValue,
  pub variant_options: Vec<VariantOption>,
  pub customizations: Vec<Customization>,
  pub image_url: String,
  // TODO: I think this can also be an enum
  pub line_item_type: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MonetaryValue {
  // ISO 4217 currency code string.
  pub value: String,
  // Monetary amount with 1,000,000 limit and no markers for the dollar amount.
  // Conforms to the selected ISO currency's precision.
  // (e.g., JPY uses 123, but USD uses 123.00 or 123)
  pub currency: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VariantOption {
  pub value: String,
  pub option_name: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Customization {
  pub label: String,
  pub value: String,
}
