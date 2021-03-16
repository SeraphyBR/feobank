mod account;
mod session;

use sqlx::SqlitePool;
use tokio::net::{TcpListener, TcpStream};
use dotenv::dotenv;

use session::Session;

#[tokio::main]
async fn main() {
    // Load .env
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    // Create a pool to sqlite database
    let pool = SqlitePool::connect(&database_url).await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _addr) = listener.accept().await.unwrap();
        let db = pool.clone();

        tokio::spawn(async move {
            handle_connection(socket, &db).await;
        });
    }
}

async fn handle_connection(stream: TcpStream, db: &SqlitePool) {
    let mut session = Session::new(stream, db.clone());
    let cpf = String::new();
    let password = String::new();
    match session.login(cpf, password) {
        Ok(_) => session.start(),
        Err(description) => {

        }
    }
}
