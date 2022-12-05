use std::io::stdin;

use hello::{hello_client::HelloClient, HelloRequest};
use tonic::transport::Channel;
use voting::{voting_client::VotingClient, VotingRequest};

pub mod voting {
    tonic::include_proto!("voting");
}
pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut voting_client = VotingClient::connect("http://[::1]:8000").await?;
    let mut hello_client = HelloClient::connect("http://[::1]:8000").await?;

    loop {
        println!("\nPlease select a service:");
        println!("Hello Service: h");
        println!("Voting Service: v");
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();

        match s.trim().to_lowercase().chars().next().unwrap() {
            'h' => hello_handler(&mut hello_client).await?,
            'v' => voting_handler(&mut voting_client).await?,
            _ => break,
        };
    }
    Ok(())
}

async fn hello_handler(
    client: &mut HelloClient<Channel>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut name = String::new();
    println!("Enter your name: ");
    stdin().read_line(&mut name).unwrap();
    let name = String::from(name.trim());
    let request = tonic::Request::new(HelloRequest { name: name });
    let response = client.hello(request).await?;
    println!("Response from server: {}", response.into_inner().message);
    Ok(())
}

async fn voting_handler(
    client: &mut VotingClient<Channel>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut u = String::new();
    let mut vote: String = String::new();
    println!("Please provide a url: ");

    stdin().read_line(&mut u).unwrap();
    let u = u.trim();

    println!("Please vote (d)own or (u)p: ");
    stdin().read_line(&mut vote).unwrap();
    let v = match vote.trim().to_lowercase().chars().next().unwrap() {
        'u' => 0,
        'd' => 1,
        _ => 1,
    };

    let request = tonic::Request::new(VotingRequest {
        url: String::from(u),
        vote: v,
    });
    let response = client.vote(request).await?;
    println!("Got: {} from service", response.into_inner().confirmation);
    Ok(())
}
