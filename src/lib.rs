pub mod error;
pub mod gateways;
pub mod options;

pub use error::PaymentError;
pub use gateways::GatewayRegistry;
pub use options::PaymentOptions;

pub fn init_logger() {
    env_logger::init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gateways::razorpay::RazorpayGateway;
    use mockito::mock;

    // Test Razorpay success scenario
    #[tokio::test]
    async fn test_razorpay_checkout_success() {
        crate::init_logger();

        let _m = mock("POST", "/v1/orders")
            .with_status(200)
            .with_body(r#"{"id":"order_id"}"#)
            .create();

        let razorpay_gateway = RazorpayGateway::new("".to_string(), "secret".to_string());
        let options = PaymentOptions {
            payment_method: None,
            receipt: Some("receipt#123".to_string()),
            customer_id: None,
            metadata: None,
        };
        let result = razorpay_gateway.checkout(500, "INR", &options).await;
        assert!(result.is_ok());
    }

    // Test Razorpay failure scenario
    #[tokio::test]
    async fn test_razorpay_checkout_failure() {
        crate::init_logger();

        let _m = mock("POST", "/v1/orders")
            .with_status(500)
            .with_body(r#"{"error":"some_error"}"#)
            .create();

        let gateway = RazorpayGateway::new("test_key".to_string(), "test_secret".to_string());
        let options = PaymentOptions {
            payment_method: None,
            receipt: Some("receipt#123".to_string()),
            customer_id: None,
            metadata: None,
        };
        let result = gateway.checkout(5000, "INR", &options).await;
        assert!(result.is_err());
    }

    // Future tests for other gateways or the registry can be added here...
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::gateways::razorpay::RazorpayGateway;
//     // use gateways::cashfree::CashfreeGateway;
//     use mockito::mock;

//     // #[tokio::test]
//     // async fn test_stripe_checkout_success() {
//     //     crate::init_logger();

//     //     let _m =mock ("POST", "/v1/charges")
//     //         .with_status(200)
//     //         .with_body(r#"{"id":"charge_id"}"#)
//     //         .create();

//     //     let gateway = StripeGateway::new("test_key".to_string());
//     //     let options = PaymentOptions {
//     //         payment_method: Some("pm_card_visa".to_string()),
//     //         receipt: None,
//     //         customer_id: None,
//     //         metadata: None,
//     //     };
//     //     let result = gateway.checkout(5000, "usd", &options).await;
//     //     assert!(result.is_ok());
//     // }

//     #[tokio::test]
//     async fn test_razorpay_checkout_success() {
//         crate::init_logger();

//         let _m = mock("POST", "/v1/orders")
//             .with_status(200)
//             .with_body(r#"{"id":"order_id"}"#)
//             .create();

//         let gateway = RazorpayGateway::new("".to_string(), "secret".to_string());
//         let options = PaymentOptions {
//             payment_method: None,
//             receipt: Some("receipt#123".to_string()),
//             customer_id: None,
//             metadata: None,
//         };
//         let result = gateway.checkout(500, "INR", &options).await;
//         assert!(result.is_ok());
//     }

//     // #[tokio::test]
//     // async fn test_cashfree_checkout_success() {
//     //     crate::init_logger();

//     //     let _m = mock("POST", "/v1/orders")
//     //         .with_status(200)
//     //         .with_body(r#"{"id":"order_id"}"#)
//     //         .create();

//     //     let gateway = CashfreeGateway::new("test_key".to_string(), "test_secret".to_string());
//     //     let options = PaymentOptions {
//     //         payment_method: None,
//     //         receipt: Some("receipt#123".to_string()),
//     //         customer_id: None,
//     //         metadata: None,
//     //     };
//     //     let result = gateway.checkout(500, "INR", &options).await;
//     //     assert!(result.is_ok());
//     // }

//     // #[tokio::test]
//     // async fn test_razorpay_checkout_failure() {
//     //     crate::init_logger();

//     //     let _m = mock("POST", "/v1/orders")
//     //         .with_status(500)
//     //         .with_body(r#"{"id":"order_id"}"#)
//     //         .create();

//     //     let gateway = RazorpayGateway::new("test_key".to_string(), "test_secret".to_string());
//     //     let options = PaymentOptions {
//     //         payment_method: None,
//     //         receipt: Some("receipt#123".to_string()),
//     //         customer_id: None,
//     //         metadata: None,
//     //     };
//     //     let result = gateway.checkout(5000, "INR", &options).await;
//     //     assert!(result.is_err());
//     // }

//     // #[tokio::test]
//     // async fn test_gateway_registry() {
//     //     crate::init_logger();

//     //     let mut registry = GatewayRegistry::new();
//     //     registry.register("stripe", "test_key".to_string());
//     //     registry.register("razorpay", "test_key".to_string());
//     //     registry.register("razorpay", "test_secret".to_string());
//     //     registry.register("cashfree", "test_key".to_string());
//     //     registry.register("cashfree", "test_secret".to_string());
//     // }

//     // #[tokio::test]
//     // async fn test_gateway_registry_checkout() {
//     //     crate::init_logger();

//     //     let mut registry = GatewayRegistry::new();
//     //     registry.register("stripe", "test_key".to_string());
//     //     registry.register("razorpay", "test_key".to_string());
//     //     registry.register("razorpay", "test_secret".to_string());
//     //     registry.register("cashfree", "test_key".to_string());
//     //     registry.register("cashfree", "test_secret".to_string());

//     //     let result = registry.checkout("stripe", 5000, "usd", &PaymentOptions::default()).await;
//     //     assert!(result.is_ok());
//     // }

//     // #[tokio::test]
//     // async fn test_gateway_registry_checkout_fail() {
//     //     crate::init_logger();

//     //     let mut registry = GatewayRegistry::new();
//     //     registry.register("stripe", "test_key".to_string());
//     //     registry.register("razorpay", "test_key".to_string());
//     //     registry.register("razorpay", "test_secret".to_string());
//     //     registry.register("cashfree", "test_key".to_string());
//     //     registry.register("cashfree", "test_secret".to_string());

//     //     let result = registry.checkout("cashfree", 5000, "usd", &PaymentOptions::default()).await;
//     //     assert!(result.is_err());
//     // }
// }
