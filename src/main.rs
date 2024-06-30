use bubble_backend::prelude::*;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:8060").expect("Failed to bind to random port");
    let server = start_server(listener).await?;

    server.await
}
