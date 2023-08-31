use std::net::TcpListener;
use sqlx::{PgPool};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = get_configuration().expect("Failed to read config");
    let address = format!("127.0.0.1:{}", conf.app_port);
    let connection_string = conf.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");
    let listener = TcpListener::bind(address).expect("failed to bind address");
    run(listener, connection_pool)?.await
}
