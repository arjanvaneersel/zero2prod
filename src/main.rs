use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind(format!("0.0.0.0:{}", configuration.application_port))
        .expect("Failed to bind random port");
    run(listener, connection)?.await
}
