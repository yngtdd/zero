use std::net::TcpListener;
use zero::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = TcpListener::bind("127.0.0.1:0");
    run(address?)?.await
}
