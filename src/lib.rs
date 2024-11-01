use actix_web::{
    dev::Server, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: std::net::TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/health_check", web::get().to(health_check))
            .route("/hey", web::get().to(manual_hello))
    })
    .listen(listener)?
    // .bind(("127.0.0.1", port))?
    .run();

    Ok(server)
}
