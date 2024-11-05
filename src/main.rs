use email_newsletter::configuration::get_configuration;
use email_newsletter::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let lst = std::net::TcpListener::bind(address).expect("Failed to bind listener");
    run(lst)?.await
}
