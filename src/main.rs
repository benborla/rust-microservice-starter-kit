use rust_microservice_starter_kit::run_app;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    match run_app().await {
        Ok(app) => {
            let listener = tokio::net::TcpListener::bind("[::]:3380").await.unwrap();
            axum::serve(listener, app.into_make_service())
                .await
                .unwrap();
        }
        Err(e) => {
            eprintln!("Application error: {}", e);
            std::process::exit(1);
        }
    }
}
