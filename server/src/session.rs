use sqlx::SqlitePool;
use tokio::net::TcpStream;

use super::account::Account;

pub struct Session {
    account: Option<Account>,
    conn: TcpStream,
    db: SqlitePool
}

impl Session {

    pub fn new(conn: TcpStream, db: SqlitePool) -> Session {
        Session {
            account: None,
            conn,
            db
        }
    }

    pub fn login(&mut self, cpf: String, password: String) -> Result<(), String> {
        todo!()
    }

    pub fn start(&mut self) {
        // Responder ao cliente que a sessão foi iniciada, logado com sucesso
        // Iniciar o loop de ação principal
        todo!()
    }
}