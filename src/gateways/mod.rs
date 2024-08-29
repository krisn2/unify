use crate::error::PaymentResult;
use crate::options::PaymentOptions;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait PaymentGateway: Send + Sync {
    async fn checkout(
        &self,
        amount: u64,
        currency: &str,
        options: &PaymentOptions,
    ) -> PaymentResult<String>;
}

pub struct GatewayRegistry {
   gateways: HashMap<String,Box<dyn PaymentGateway>>,
}

impl GatewayRegistry {
    pub fn new() -> Self {
        GatewayRegistry {
            gateways: HashMap::new(),
        }
    }

    pub fn register_gateway(&mut self, name: String, gateway: Box<dyn PaymentGateway>) {
        self.gateways.insert(name, gateway);
    }

    pub async fn checkout(
        &self,
        name: &str,
        amount: u64,
        currency: &str,
        options: &PaymentOptions,
    ) -> PaymentResult<String> {
        if let Some(gateway) = self.gateways.get(name) {
            gateway.checkout(amount, currency, options).await
        } else {
            Err(crate::error::PaymentError::Unknown)
        }
    }
}

pub mod cashfree;
pub mod razorpay;
pub mod stripe;
