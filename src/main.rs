use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("msg");
    println!("config, {:?}", config.database);
    let listener = TcpListener::bind("127.0.0.1:8000").expect("failed to listen");
    run(listener)?.await
}
