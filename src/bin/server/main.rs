
use std::io::ErrorKind;

use sqlx::SqlitePool;
use tokio::net::TcpListener;
use dotenv::dotenv;

mod session;
use session::Session;
use tracing::{info, warn};

use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Inicialize default log subscriber
    tracing_subscriber::fmt::init();

    // Load .env
    dotenv().ok();

    let addr = std::env::var("SERVER_ADDR").expect("SERVER_ADDR must be set!");
    let database_uri = std::env::var("DATABASE_URL").expect("DATABASE_URI must be set!");

    // Bind the listener to the address
    let listener = TcpListener::bind(&addr).await.unwrap();

    // Create a pool to sqlite database
    let pool = SqlitePool::connect(&database_uri).await.unwrap();

    info!("Started server in addr: {}", addr);

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, addr) = match listener.accept().await {
            Ok(c) => {
                info!("Connected to {:?}", c.1);
                c
            },
            Err(e) => {
                warn!("{}", e);
                continue;
            }
        };

        let db = pool.clone();

        // One Tokio Task By Connection/Session
        tokio::spawn(async move {
            let mut session = Session::new(socket, db);
            session.start()
                .await
                .unwrap_or_else(|e| {
                    if e.kind() == ErrorKind::UnexpectedEof {
                        info!("The connection to {:?} was terminated unexpectedly", addr)
                    }
                });
            info!("The session with {:?} ended", addr);
        });
    }
}