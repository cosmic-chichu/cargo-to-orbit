use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        // chiq: read about mod crate and what into does
        name: "chichu".to_string(),
        // name: "chichu".into(),
    });

    // chiq: what does await do? Read the async book for details on how async/await and executors work
    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}