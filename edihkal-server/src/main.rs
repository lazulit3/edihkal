use axum::Server;
use edihkal_server::{configuration::get_configuration, router::router};
use sqlx::{Connection, PgConnection};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to database");
    let addr = SocketAddr::from(([127, 0, 0, 1], configuration.application_port));

    Server::bind(&addr)
        .serve(router(connection).into_make_service())
        .await
        .unwrap();
}
