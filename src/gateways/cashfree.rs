use async_trait::async_trait;
use reqwest::Client;
use crate::error::{PaymentError, PaymentResult};
use crate::options::PaymentOptions;
use super::PaymentGateway;

pub struct CashfreeGateway {
    key_id: String,
    key_secret: String,
}

impl CashfreeGateway {
    pub fn new(key_id: String, key_secret: String) -> Self {
        CashfreeGateway { key_id, key_secret}
    }
}

#[async_trait]
impl PaymentGateway for CashfreeGateway {
    async fn checkout(&self, amount:u64, currency: &str, options: &PaymentOptions)-> PaymentResult<String> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|e| PaymentError::NetworkError(e.to_string()))?;

        let response = client
            .post(" https://sandbox.cashfree.com/pg/orders")
            .basic_auth(&self.key_id, Some(&self.key_secret))
            .json(&serde_json::json!({
                "amount": amount ,
                "currency": currency,
                "receipt": options.receipt.as_ref().unwrap_or(&"".to_string()),
                "payment_capture": 1,
            }))
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

