use super::PaymentGateway;
use crate::error::{PaymentError, PaymentResult};
use crate::options::PaymentOptions;
use async_trait::async_trait;
use reqwest::Client;

pub struct StripeGateway {
    api: String,
    base_url: String,
}

impl StripeGateway {
    pub fn new(api: String, base_url: String) -> Self {
        Self {
            api,
            base_url,  
        }
    }
}
#[async_trait]
impl PaymentGateway for StripeGateway {
    async fn checkout(
        &self,
        amount: u64,
        currency: &str,
        options: &PaymentOptions,
    ) -> PaymentResult<String> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|e| PaymentError::NetworkError(e.to_string()))?;

        let response = client
            .post(&format!("{}/v1/orders", self.base_url))
            .basic_auth(&self.api, Some(""))
            .form(&[
                ("amount", amount.to_string()),
                ("currency", currency.to_string()),
                (
                    "payment_method",
                    options.payment_method.clone().unwrap_or_default(),
                ),
                ("confirmation_method", "automatic".to_string()),
                ("confirm", "true".to_string()),
            ])
            .send()
            .await
            .map_err(|e| PaymentError::NetworkError(e.to_string()))?;

        if response.status().is_success() {
            let body = response
                .text()
                .await
                .map_err(|e| PaymentError::NetworkError(e.to_string()))?;
            Ok(body)
        } else {
            Err(PaymentError::ApiError(
                response
                    .text()
                    .await
                    .map_err(|e| PaymentError::NetworkError(e.to_string()))?,
            ))
        }
    }
}
