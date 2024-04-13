use proto::admin_server::{Admin, AdminServer};
use proto::calculator_server::{Calculator, CalculatorServer};
use tonic::transport::Server;
mod proto {
    tonic::include_proto!("calculator");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

type State = std::sync::Arc<tokio::sync::RwLock<u64>>;

#[derive(Debug, Default)]
struct CalculatorService {
    state: State,
}

impl CalculatorService {
    async fn increment_counter(&self) {
        let mut count = self.state.write().await;
        *count += 1;
        println!("Request count: {}", *count);
    }
}

fn process_operation(
    operation: &str,
    a: i64,
    b: i64,
) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
    match operation {
        "add" => Ok(tonic::Response::new(proto::CalculationResponse {
            result: a + b,
        })),
        "subtract" => Ok(tonic::Response::new(proto::CalculationResponse {
            result: a - b,
        })),
        "multiply" => Ok(tonic::Response::new(proto::CalculationResponse {
            result: a * b,
        })),
        "divide" => {
            if b == 0 {
                Err(tonic::Status::invalid_argument("Cannot divide by zero!"))
            } else {
                Ok(tonic::Response::new(proto::CalculationResponse {
                    result: a / b,
                }))
            }
        }
        _ => Err(tonic::Status::invalid_argument("Invalid operation")),
    }
}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
        let input = request.get_ref();
        self.increment_counter().await;

        process_operation("add", input.a, input.b)
    }
    async fn divide(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
        let input = request.get_ref();
        self.increment_counter().await;
        if input.b == 0 {
            return Err(tonic::Status::invalid_argument("Cannot divide by zero!"));
        }

        process_operation("divide", input.a, input.b)
    }
    async fn subtract(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
        let input = request.get_ref();
        self.increment_counter().await;

        process_operation("subtract", input.a, input.b)
    }
    async fn multiply(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
        self.increment_counter().await;

        let input = request.get_ref();

        process_operation("multiply", input.a, input.b)
    }
}

#[derive(Debug, Default)]
struct AdminService {
    state: State,
}
#[tonic::async_trait]
impl Admin for AdminService {
    async fn get_request_count(
        &self,
        _request: tonic::Request<proto::GetCountRequest>,
    ) -> Result<tonic::Response<proto::CounterResponse>, tonic::Status> {
        let count = self.state.read().await;
        let response = proto::CounterResponse { count: *count };

        Ok(tonic::Response::new(response))
    }
}

use tonic::metadata::MetadataValue;
use tonic::{Request, Status};

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "Bearer token".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("Token inválido")),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let state = State::default();

    let calc = CalculatorService {
        state: state.clone(),
    };
    let admin = AdminService {
        state: state.clone(),
    };

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .accept_http1(true)
        .layer(tower_http::cors::CorsLayer::permissive())
        .add_service(service)
        .add_service(tonic_web::enable(CalculatorServer::new(calc)))
        .add_service(AdminServer::with_interceptor(admin, check_auth))
        .serve(addr)
        .await?;

    Ok(())
}
