use rss::startup::run;
use std::net::TcpListener;
use rss::configuration::get_configuration;
use rss::telemetry::{get_subscriber, init_subscriber};

use sqlx::PgPool;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let subscriber = get_subscriber("rss".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}