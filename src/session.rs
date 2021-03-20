use rust_decimal::Decimal;
use sqlx::SqlitePool;
use tokio::{io::{AsyncReadExt, AsyncWriteExt, BufWriter}, net::TcpStream};
use crate::account::{AccountAction, NewAccount};
use crate::account::AccountAction::*;

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

    pub async fn start(&mut self) {
        let mut data = String::new();
        self.conn.read_to_string(&mut data).await.unwrap();

        match serde_json::from_str::<AccountAction>(&data) {
            Ok(action) => self.take_action(action),
            Err(e) => {}
        };

        // Responder ao cliente que a sessão foi iniciada, logado com sucesso
        let data = serde_json::to_string(&self.account).unwrap();
        self.conn.write_all(data.as_bytes()).await.unwrap();

        // Iniciar o loop de ação principal
        todo!()
    }

    fn take_action(&mut self, action: AccountAction) {
        match action {
            Login { cpf, password } => self.login(cpf, password),
            CreateAccount(data) => self.create_account(data),
            DeleteAccount => self.delete_account(),
            TransferMoney { dest_cpf, value } => self.transfer_money(dest_cpf, value),
            PayBill(_) => {}
            CreateBill {  } => {}
            GetStatment => {}
        }
    }

    fn login(&mut self, cpf: String, password: String) {
        todo!()
    }

    fn create_account(&mut self, data: NewAccount) {
        todo!()
    }

    fn delete_account(&mut self) {
        todo!()
    }

    fn transfer_money(&mut self, dest_cpf: String, value: Decimal) {
        todo!()
    }
}