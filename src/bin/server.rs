
use sqlx::SqlitePool;
use tokio::net::{TcpListener, TcpStream};
use dotenv::dotenv;

use feobank::session::Session;
use tracing::{info, warn};

#[tokio::main]
async fn main() {
    // Load .env
    dotenv().ok();

    let addr = std::env::var("SERVER_ADDR").expect("SERVER_ADDR must be set!");
    let database_uri = std::env::var("DATABASE_URI").expect("DATABASE_URI must be set!");

    // Bind the listener to the address
    let listener = TcpListener::bind(&addr).await.unwrap();

    // Create a pool to sqlite database
    let pool = SqlitePool::connect(&database_uri).await.unwrap();

    info!("Started server in addr: {}", addr);

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _addr) = match listener.accept().await {
            Ok(c) => {
                info!("Connected to ");
                c
            },
            Err(e) => {
                warn!("");
                continue;
            }
        };

        let db = pool.clone();

        tokio::spawn(async move {
            let mut session = Session::new(socket, db);
            session.start().await;
        });
    }
}