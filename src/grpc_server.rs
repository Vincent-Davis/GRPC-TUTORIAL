pub mod services {
    tonic::include_proto!("services");
}


use tonic::{transport::Server, Request, Response, Status};

use services::payment_service_server::{PaymentService, PaymentServiceServer};
use services::{PaymentRequest, PaymentResponse};


#[derive(Default)]
pub struct MyPaymentService {}

#[tonic::async_trait]
impl PaymentService for MyPaymentService {
    async fn process_payment(
        &self,
        request: Request<PaymentRequest>,
    ) -> Result<Response<PaymentResponse>, Status> {
        println!("Received payment request: {:?}", request);

        // Process the request and return a response
        Ok(Response::new(PaymentResponse { success: true }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // binding address dan port
    let addr = "[::1]:50051".parse()?;

    // inisialisasi implementasi service
    let payment_service = MyPaymentService::default();

    // build dan start gRPC server
    Server::builder()
        .add_service(PaymentServiceServer::new(payment_service))
        .serve(addr)
        .await?;

    Ok(())
}