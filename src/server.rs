use tonic::{transport::Server,Request,Response,Status};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::{
    greeter_server::{Greeter,GreeterServer},
    HelloRequest,HelloReply,
};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for  MyGreeter {
    // add code here
    async fn say_hello(&self,request: Request<HelloRequest>) -> Result<Response<HelloReply>,Status> {
        println!("Got a request: {:?}", request);
        let reply = hello_world::HelloReply{
            message: format!("Hello {}!",request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    
    let addr = "[::0]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    Server::builder().add_service(GreeterServer::new(greeter)).serve(addr).await?;

    Ok(())
}