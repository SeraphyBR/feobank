use rust_decimal::Decimal;
use sqlx::SqlitePool;
use tokio::{io::{self, AsyncReadExt, AsyncWriteExt}, net::TcpStream};
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

    async fn write_message(&mut self, message: String) -> io::Result<()> {
        let lenght = message.len();
        self.conn.write_all(&lenght.to_le_bytes()).await?;
        self.conn.write_all(message.as_bytes()).await?;
        Ok(())
    }

    async fn read_message(&mut self) -> io::Result<String> {
        let mut lenght = [0u8; std::mem::size_of::<usize>()];
        self.conn.read_exact(&mut lenght).await?;
        let lenght = usize::from_le_bytes(lenght);

        let mut buf = vec![0u8; lenght];
        self.conn.read_exact(&mut buf).await?;

        Ok(String::from_utf8_lossy(&buf).into())
    }

    pub async fn start(&mut self) -> io::Result<()>{
        loop {
            let action = self.read_message().await?;
            match serde_json::from_str::<AccountAction>(&action) {
                Ok(action) => self.take_action(action).await?,
                Err(e) => {}
            };
        }
    }

    async fn take_action(&mut self, action: AccountAction) -> io::Result<()> {
        match action {
            Login { cpf, password } => self.login(cpf, password).await?,
            CreateAccount(data) => self.create_account(data).await,
            DeleteAccount => self.delete_account().await,
            TransferMoney { dest_cpf, value } => self.transfer_money(dest_cpf, value).await,
            PayBill(_) => {}
            CreateBill {  } => {}
            GetStatment => {}
        }
        Ok(())
    }

    async fn login(&mut self, cpf: String, password: String) -> io::Result<()> {
        // Responder ao cliente que a sessão foi iniciada, logado com sucesso
        let data = serde_json::to_string(&self.account).unwrap();
        self.write_message(data).await?;
        self.write_message("Senha incorreta".to_string()).await?;
        Ok(())
    }

    async fn create_account(&mut self, data: NewAccount) {

    }

    async fn delete_account(&mut self) {

    }

    async fn transfer_money(&mut self, dest_cpf: String, value: Decimal) {

    }
}