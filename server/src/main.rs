use hello::{
    hello_server::{Hello, HelloServer},
    HelloRequest, HelloResponse,
};
use tonic::{transport::Server, Request, Response, Status};
use voting::{
    voting_server::{Voting, VotingServer},
    VotingRequest, VotingResponse,
};

pub mod voting {
    tonic::include_proto!("voting");
}

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Debug, Default)]
pub struct VotingService {}

#[tonic::async_trait]
impl Voting for VotingService {
    async fn vote(
        &self,
        request: Request<VotingRequest>,
    ) -> Result<Response<VotingResponse>, Status> {
        let r = request.into_inner();
        match r.vote {
            0 => Ok(Response::new(voting::VotingResponse {
                confirmation: { format!("Happy to confirm that you upvoted for {}", r.url) },
            })),
            1 => Ok(Response::new(voting::VotingResponse {
                confirmation: { format!("Confirmation that you downvoted for {}", r.url) },
            })),
            _ => Err(Status::new(
                tonic::Code::OutOfRange,
                "Invalid vote provided",
            )),
        }
    }
}

#[derive(Debug, Default)]
pub struct HelloService {}

#[tonic::async_trait]
impl Hello for HelloService {
    async fn hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let r = request.into_inner();
        Ok(Response::new(hello::HelloResponse {
            message: { format!("hello, {}", r.name) },
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<HelloServer<HelloService>>()
        .await;
    health_reporter
        .set_serving::<VotingServer<VotingService>>()
        .await;

    let addr = "[::1]:8000".parse().unwrap();
    let voting_service = VotingService::default();
    let hello_service = HelloService::default();

    println!("Listening on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(VotingServer::new(voting_service))
        .add_service(HelloServer::new(hello_service))
        .serve(addr)
        .await?;

    Ok(())
}
