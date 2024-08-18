use unify::{init_logger, PaymentOptions, GatewayRegistry};
use unify::gateways::{stripe::StripeGateway, razorpay::RazorpayGateway, cashfree::CashfreeGateway};

#[tokio::main]
async fn main() {
    init_logger();

    let stripe_gateway = StripeGateway::new("your_stripe_secret_key".to_string());
    let razorpay_gateway = RazorpayGateway::new("your_razorpay_key_id".to_string(), "your_razorpay_key_secret".to_string());
    let cashfree_gateway = CashfreeGateway::new("your_cashfree_key_id".to_string(), "your_cashfree_key_secret".to_string());

    let mut registry = GatewayRegistry::new();
    registry.register_gateway("stripe".to_string(), Box::new(stripe_gateway));
    registry.register_gateway("razorpay".to_string(), Box::new(razorpay_gateway));
    registry.register_gateway("cashfree".to_string(), Box::new(cashfree_gateway));

    let options = PaymentOptions {
        payment_method: Some("pm_card_visa".to_string()),
        receipt: Some("receipt#123".to_string()),
    };

    match registry.checkout("stripe", 5000, "usd", &options).await {
        Ok(response) => println!("Stripe Payment Response: {:?}", response),
        Err(error) => eprintln!("Error: {:?}", error),
    }

    match registry.checkout("razorpay", 5000, "INR", &options).await {
        Ok(response) => println!("Razorpay Payment Response: {:?}", response),
        Err(error) => eprintln!("Error: {:?}", error),
    }

    match registry.checkout("cashfree", 5000, "INR", &options).await {
        Ok(response) => println!("Cashfree Payment Response: {:?}", response),
        Err(error) => eprintln!("Error: {:?}", error),
    }
}
