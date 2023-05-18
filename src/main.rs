use std::net::TcpListener;
use zero2prod::{startup::run, configuration::get_configuration};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", configuration.application_port)).expect("Failed to bind random port");
    run(listener)?.await
}