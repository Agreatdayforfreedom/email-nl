use crate::routes::{health_check, subscriptions};
use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

pub fn run(listener: std::net::TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check::health_check))
            .route("/subscriptions", web::post().to(subscriptions::subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
