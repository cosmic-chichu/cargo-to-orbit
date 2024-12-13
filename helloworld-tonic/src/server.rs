use tonic::{transport::Server, Request, Response, Status};

// chiq: How do i know that tonic generated greeter_server and all the structs?
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

// chiq: read about modules
pub mod hello_world {
    tonic::include_proto!("helloworld");
}

// chiq: read about derive. Also why no semi-colon? because its an expression?
#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    // chiq: how would we know the signature of this function?
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        // chiq: read about into and inner
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    // chiq: what does default derive provide?
    let greeter = MyGreeter::default();


    // chiq: what does adding ? at the end of a call do?
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}