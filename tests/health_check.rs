use std::net::TcpListener;

#[actix_web::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let lst = TcpListener::bind("127.0.0.1:0").expect("Failed to bind listener");

    let port = lst.local_addr().unwrap().port();

    let server = email_newsletter::run(lst).expect("Failed to spawn out app.");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
