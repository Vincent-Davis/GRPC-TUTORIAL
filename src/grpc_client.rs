// 1. Include the generated protobuf code
pub mod services {
    tonic::include_proto!("services");
}

use services::payment_service_client::PaymentServiceClient;
use services::PaymentRequest;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 2. Connect to our local server
    let mut client = PaymentServiceClient::connect("http://[::1]:50051").await?;

    // 2. Create the gRPC request wrapper with our payload
    let request = Request::new(PaymentRequest {
        user_id: "user-123".to_string(),
        amount: 100.0,
    });

    // 3. Send it and await the response
    let response = client.process_payment(request).await?;

    // 4. Unwrap and print the inner message
    println!("RESPONSE={:?}", response.into_inner());

    Ok(())
}