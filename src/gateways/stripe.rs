use async_trait::async_trait;
use reqwest::Client;
use crate::error::{PaymentError, PaymentResult};
use crate::options::PaymentOptions;
use super::PaymentGateway;

pub struct StripeGateway {
    api_key: String,
}

impl StripeGateway {
    pub fn new(api_key: String) -> Self {
        StripeGateway { api_key }
    }
}

#[async_trait]
impl PaymentGateway for StripeGateway {
    async fn checkout(&self, amount: u64, currency: &str, options: &PaymentOptions) -> PaymentResult<String> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|e| PaymentError::NetworkError(e.to_string()))?;

        let response = client
            .post("https://api.stripe.com/v1/charges")
            .basic_auth(&self.api_key, Some(""))
            .form(&[
                ("amount", amount.to_string()),
                ("currency", currency.to_string()),
                ("payment_method", options.payment_method.clone().unwrap_or_default()),
                ("confirmation_method", "automatic".to_string()),
                ("confirm", "true".to_string()),
            ])
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(e.to_string()))?;

        if response.status().is_success() {
            let body = response.text().await.map_err(|e| PaymentError::NetworkError(e.to_string()))?;
            Ok(body)
        } else {
            Err(PaymentError::ApiError(response.text().await.map_err(|e| PaymentError::NetworkError(e.to_string()))?))
        }
    }
}
