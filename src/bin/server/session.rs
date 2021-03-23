use rust_decimal::Decimal;
use sqlx::SqlitePool;
use tokio::{io::{AsyncReadExt, AsyncWriteExt, BufWriter}, net::TcpStream};
use feobank::account::{AccountAction, NewAccount};
use feobank::account::AccountAction::*;

use feobank::account::Account;

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

    async fn write_message(&mut self, message: String) {
        let lenght = message.len();
        self.conn.write_all(&lenght.to_le_bytes()).await.unwrap();
        self.conn.write_all(message.as_bytes()).await.unwrap();
    }

    async fn read_message(&mut self) -> String {
        let mut lenght = [0u8; std::mem::size_of::<usize>()];
        self.conn.read_exact(&mut lenght).await.unwrap();
        let lenght = usize::from_le_bytes(lenght);

        let mut buf = vec![0u8; lenght];
        self.conn.read_exact(&mut buf).await.unwrap();
        String::from_utf8_lossy(&buf).into()
    }

    pub async fn start(&mut self) {
        let action = self.read_message().await;
        println!("Dados recebidos: {}", action);

        match serde_json::from_str::<AccountAction>(&action) {
            Ok(action) => self.take_action(action),
            Err(e) => {}
        };

        // Responder ao cliente que a sessão foi iniciada, logado com sucesso
        let data = serde_json::to_string(&self.account).unwrap();
        self.write_message(data).await;

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