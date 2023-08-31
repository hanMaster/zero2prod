use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = get_configuration().expect("Failed to read config");
    let address = format!("127.0.0.1:{}", conf.app_port);
    let listener = TcpListener::bind(address).expect("failed to bind address");
    run(listener)?.await
}
