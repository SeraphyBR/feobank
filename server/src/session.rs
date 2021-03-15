use tokio::net::TcpStream;

use super::account::Account;

pub struct Session {
    account: Account,
    conn: TcpStream
}

impl Session {
    pub fn login(cpf: String, password: String) -> Result<Session, String> {
        todo!()
    }

    pub fn start(&mut self, stream: TcpStream) {
        self.conn = stream;
        todo!()
    }
}