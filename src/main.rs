use email_newsletter::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let lst = std::net::TcpListener::bind("127.0.0.1:8080").expect("Failed to bind listener");
    run(lst)?.await
}