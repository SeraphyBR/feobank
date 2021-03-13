use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            handle_connection(socket).await;
        });

    }
}

async fn handle_connection(socket: TcpStream) {

}
