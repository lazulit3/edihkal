use std::net::TcpListener;

use axum::Server;
use once_cell::sync::Lazy;
use sea_orm::DatabaseConnection;
use tower::make::Shared;

use edihkal::{app::router, tracing::configure_tracing};

use super::unique_database;

// Ensure that `tracing` is only initialized once.
static TRACING: Lazy<()> = Lazy::new(|| {
    // Use TEST_LOG to determine whether test logs should output.
    if std::env::var("TEST_LOG").is_ok() {
        configure_tracing("test", "info", std::io::stdout);
    } else {
        configure_tracing("test", "info", std::io::sink);
    }
});

/// Contains details for connecting to an edihkal service and database used
/// for integration testing.
pub struct TestService {
    db_conn: DatabaseConnection,
    service_url: String,
}

impl TestService {
    /// Create a local edihkal service and database for use in testing.
    ///
    /// The edihkal service is bound to http://127.0.0.1 on an unused port.
    /// The database connection points to a freshly created/migrated database with a unique name
    /// using the environment's configuration for connecting to a database.
    pub async fn new() -> TestService {
        // Initialize `tracing` only if not already initialized.
        Lazy::force(&TRACING);

        let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind ephemeral socket");
        let service_addr = listener.local_addr().unwrap();
        let service_url = format!("http://{}", service_addr);
        println!("Edihkal is listening on {}", service_url);

        let db_conn = unique_database().await;
        let router = router(db_conn.clone());

        tokio::spawn(async move {
            let server = Server::from_tcp(listener).unwrap().serve(Shared::new(router));
            server.await.expect("server error");
        });

        TestService {
            db_conn,
            service_url,
        }
    }

    /// Returns the URL that the edihkal service is listening on.
    pub fn service_url(&self) -> &str {
        &self.service_url
    }

    /// Returns a `DatabaseConnection` to the unique database created for this edihkal test service.
    pub fn database_connection(&self) -> &DatabaseConnection {
        &self.db_conn
    }
}
