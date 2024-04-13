use proto::calculator_server::{Calculator, CalculatorService};

mod proto {
    tnic::include_proto!("calculator");
}

#[derive(Debug, Default)]
struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self,
        request: tenic::Request<proto::CalculatorRequest>,
    ) -> Result<tonic::Response<proto::CalculatorResponse>, tonic::Status> {
        println!("Got a request: {:?}", request);

        let input = request.get_ref();

        let response = proto::CalculatorResponse {};
    }
}
fn main() {
    println!("Hello, world!");
}
