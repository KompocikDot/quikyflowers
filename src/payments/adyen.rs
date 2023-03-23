use serde::{Deserialize, Serialize};

use crate::{errors::ApiError, models::links::Link};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Amount {
    currency: String,
    value: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AdyenPaymentLinkData<'p> {
    merchant_account: String,
    reference: String,
    amount: Amount,
    blocked_payment_methods: Vec<&'p str>,
    required_shopper_fields: Vec<&'p str>,
    reusable: bool,
    shopper_locale: &'p str,
}

trait Payment {
    fn new(amount: i32) -> Self;
}

impl Payment for AdyenPaymentLinkData<'_> {
    fn new(amount: i32) -> Self {
        Self {
            merchant_account: String::from("FwsAccountECOM"),
            reference: String::from("słoik mały 45cm"),
            amount: Amount {
                currency: String::from("PLN"),
                value: amount * 100,
            },
            blocked_payment_methods: vec!["giropay", "ideal", "klarna", "paysafecard", "trustly"],
            required_shopper_fields: vec!["shopperName", "shopperEmail", "telephoneNumber"],
            reusable: true,
            shopper_locale: "pl-PL",
        }
    }
}

#[derive(Deserialize, PartialEq)]
enum PaymentLinkStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "completed")]
    Completed,
    PaymentPending,
}

#[derive(Deserialize)]
struct PaymentLinkResponse {
    status: PaymentLinkStatus,
    url: String,
}

pub async fn create_payment_link(secret: &String, link: &Link) -> Result<String, ApiError> {
    let adyen_data = AdyenPaymentLinkData::new(link.price);

    let payment_req = reqwest::Client::new()
        .post("https://checkout-test.adyen.com/v70/paymentLinks")
        .header("X-API-Key", secret)
        .json(&adyen_data)
        .send()
        .await?
        .json::<PaymentLinkResponse>()
        .await?;

    if payment_req.status == PaymentLinkStatus::Active {
        Ok(payment_req.url)
    } else {
        Err(ApiError::InternalServerError(String::from(
            "Could not create payment link",
        )))
    }
}
