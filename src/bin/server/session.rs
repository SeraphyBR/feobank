use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::SqlitePool;
use tokio::{io::{self, AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use feobank::user::*;
use feobank::user::UserAction::*;
use uuid::Uuid;

use bcrypt::{DEFAULT_COST, hash, verify};

pub struct Session {
    user: Option<User>,
    conn: TcpStream,
    db: SqlitePool
}

impl Session {

    pub fn new(conn: TcpStream, db: SqlitePool) -> Session {
        Session {
            user: None,
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
            match serde_json::from_str::<UserAction>(&action) {
                Ok(action) => self.take_action(action).await?,
                Err(e) => {}
            };
        }
    }

    async fn take_action(&mut self, action: UserAction) -> io::Result<()> {
        match action {
            Login { cpf, password } => self.login(cpf, password).await?,
            CreateUser(data) => self.create_user(data).await?,
            DeleteAccount => self.delete_user().await,
            TransferMoney { dest_cpf, value } => self.transfer_money(dest_cpf, value).await,
            PayBill(_) => {}
            CreateBill {  } => {}
            GetStatment => {}
        }
        Ok(())
    }

    async fn login(&mut self, cpf: String, password: String) -> io::Result<()> {
        let record = sqlx::query!("SELECT password FROM user WHERE cpf = ?", cpf)
            .fetch_one(&self.db).await.unwrap();

        let valid = verify(password, &record.password).unwrap();

        let response: Result<(), &str>;
        if valid {
            response = Ok(());
            let record = sqlx::query!("SELECT * FROM user WHERE cpf = ?", cpf)
                .fetch_one(&self.db).await.unwrap();

            self.user = Some(
                User {
                    id: Uuid::parse_str(&record.id).unwrap(),
                    account_id: Uuid::parse_str(&record.account_id).unwrap(),
                    cpf: record.cpf,
                    password: record.password,
                    email: record.email,
                    name: record.name,
                    address: record.address,
                    phone: record.phone,
                    birthdate: record.birthdate.date(),
                    last_login: record.last_login
                }
            );
        }
        else {
            response = Err("CPF/Password is not valid");
            self.user = None;
        }

        // Responder ao cliente que a sessão foi iniciada, logado com sucesso
        let data = serde_json::to_string(&response).unwrap();
        self.write_message(data).await?;
        Ok(())
    }

    async fn create_user(&mut self, u: NewUser) -> io::Result<()> {
        let id_account = Uuid::new_v4().to_string();
        let id_user = Uuid::new_v4().to_string();
        let password = hash(u.password, DEFAULT_COST).unwrap();
        let _result = sqlx::query!(
            "INSERT INTO account (
                id,
                agency
            ) VALUES (?, ?)",
            id_account,
            1
        ).execute(&self.db).await.unwrap();

        let birthdate = u.birthdate.and_hms(1, 0, 0);

        let _result = sqlx::query!(
            "INSERT INTO user (
                id,
                account_id,
                cpf,
                password,
                email,
                name,
                address,
                phone,
                birthdate
            ) VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?)",
            id_user,
            id_account,
            u.cpf,
            password,
            u.email,
            u.name,
            u.address,
            u.phone,
            birthdate
        )
        .execute(&self.db).await.unwrap();

        let response: Result<(), String> = Ok(());
        let message = serde_json::to_string(&response).unwrap();
        self.write_message(message).await?;
        Ok(())
    }

    async fn delete_user(&mut self) {

    }

    async fn transfer_money(&mut self, dest_cpf: String, value: Decimal) {

    }
}