mod routes;

use poem::{listener::TcpListener, Server};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let app = routes::app_routes();
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
