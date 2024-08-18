use std::collections::HashMap;

#[derive(Debug)]
pub struct PaymentOptions {
    pub payment_method: Option<String>,
    pub receipt: Option<String>,
    pub customer_id: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}
