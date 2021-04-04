use std::os::linux::raw::stat;

use chrono::NaiveDate;
use sqlx::SqlitePool;
use tokio::{io::{self, AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use feobank::{account::Account, bill::Bill, user::*};
use feobank::user::UserAction::*;
use uuid::Uuid;

use bcrypt::{DEFAULT_COST, hash, verify};

pub struct Session {
    user: Option<(User, Account)>,
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
                Ok(action) => {
                    match action {
                        CloseServerConnection => break Ok(()),
                        _ => self.take_action(action).await?
                    }
                },
                Err(e) => {}
            };
        }
    }

    async fn take_action(&mut self, action: UserAction) -> io::Result<()> {
        match action {
            Login { cpf, password } => self.login(cpf, password).await?,
            Logout => self.logout().await,
            CreateUser(data) => self.create_user(data).await?,
            DeleteUser => self.delete_user().await,
            TransferMoney { dest_cpf, value } => self.transfer_money(dest_cpf, value, None).await?,
            GetBillInfo(bill_id) => self.get_bill_info(bill_id).await?,
            PayBill(bill_id) => self.pay_bill(bill_id).await?,
            CreateBill(value) => self.create_bill(value).await?,
            GetStatment => self.gen_statment().await?,
            GetBasicInfo => self.get_basic_info().await?,
            _ => {}
        }
        Ok(())
    }

    async fn login(&mut self, cpf: String, password: String) -> io::Result<()> {
        let record = sqlx::query!("SELECT password FROM user WHERE cpf = ?", cpf)
            .fetch_one(&self.db).await.unwrap();

        let valid = verify(password, &record.password).unwrap();

        let response: Result<(), &str>;
        if valid {
            let record = sqlx::query!("SELECT * FROM user WHERE cpf = ?", cpf)
                .fetch_one(&self.db).await.unwrap();

            let user = User {
                id: Uuid::parse_str(&record.id).unwrap(),
                account_id: Uuid::parse_str(&record.account_id).unwrap(),
                cpf: record.cpf,
                password: record.password,
                email: record.email,
                name: record.name,
                address: record.address,
                phone: record.phone,
                birthdate: record.birthdate,
                last_login: record.last_login
            };

            let account_id = user.account_id.to_hyphenated();

            let record = sqlx::query!("SELECT * FROM account WHERE id = ?", account_id)
                .fetch_one(&self.db).await.unwrap();

            let account = Account {
                id: Uuid::parse_str(&record.id).unwrap(),
                agency: record.agency as u8,
                balance: record.balance,
                created_date: record.created_date
            };

            self.user = Some((user, account));
            response = Ok(());
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
                agency,
                balance
            ) VALUES (?, ?, ?)",
            id_account, 1, 100
        ).execute(&self.db).await.unwrap();

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
            u.birthdate
        )
        .execute(&self.db).await.unwrap();

        let response: Result<(), String> = Ok(());
        let message = serde_json::to_string(&response).unwrap();
        self.write_message(message).await?;
        Ok(())
    }

    async fn get_bill_info(&mut self, bill_id: Uuid) -> io::Result<()> {
        let bill_id = bill_id.to_string();
        let result = sqlx::query!("SELECT * FROM bill WHERE id = ?", bill_id)
            .fetch_one(&self.db).await;

        let response = match result {
            Ok(b) => Ok(Bill {
                id: Uuid::parse_str(&b.id).unwrap(),
                account_id: Uuid::parse_str(&b.account_id).unwrap(),
                favored_name: b.favored_name,
                value: b.value,
                created_date: b.created_date
            }),
            Err(_) => {
                Err("The bill does not exist, or has already been paid!")
            }
        };

        let message = serde_json::to_string(&response).unwrap();
        self.write_message(message).await?;
        Ok(())
    }

    async fn pay_bill(&mut self, bill_id: Uuid) -> io::Result<()> {
        if self.user.is_some() {
            let bill_id = bill_id.to_hyphenated();
            let result = sqlx::query!("SELECT account_id, value FROM bill WHERE id = ?", bill_id)
                .fetch_one(&self.db).await;

            match result {
                Ok(b) => {
                    let account_id = Uuid::parse_str(&b.account_id).unwrap();
                    let initial_balance = self.user.as_ref().unwrap().1.balance;

                    self.transfer_money(String::new(), b.value, Some(account_id)).await?;

                    let after_balance = self.user.as_ref().unwrap().1.balance;

                    if initial_balance != after_balance {
                        sqlx::query!("DELETE FROM bill WHERE id = ?", bill_id)
                            .execute(&self.db)
                            .await
                            .unwrap();
                    }
                },
                Err(_) => {
                    let response: Result<(), &str> = Err("The bill does not exist, or has already been paid!");
                    let message= serde_json::to_string(&response).unwrap();
                    self.write_message(message).await?;
                }
            };
        }
        Ok(())
    }

    async fn delete_user(&mut self) {

    }

    async fn transfer_money(&mut self, dest_cpf: String, value: f32, account_id: Option<Uuid>) -> io::Result<()> {
        if let Some((_, account)) = &mut self.user {
            let response: Result<(), &str>;

            if account.balance - value >= 0f32 {
                let remain = account.balance - value;

                let account_dist: String;

                if dest_cpf.is_empty() && account_id.is_some() {
                    account_dist = account_id.unwrap().to_string();
                }
                else {
                    let record = sqlx::query!("SELECT account_id FROM user WHERE cpf = ?", dest_cpf)
                        .fetch_one(&self.db).await;

                    let record = match record {
                        Ok(r) => r,
                        Err(_) => {
                            response = Err("User not found!");
                            let message= serde_json::to_string(&response).unwrap();
                            self.write_message(message).await?;
                            return Ok(());
                        }
                    };

                    account_dist = record.account_id;
                }

                let record = sqlx::query!("SELECT balance FROM account WHERE id = ?", account_dist)
                    .fetch_one(&self.db).await.unwrap();

                let new_balance_dist = record.balance + value;

                let id_transaction = Uuid::new_v4().to_string();
                let account_id = account.id.to_hyphenated();

                sqlx::query!("INSERT INTO transactions(id,account_src,account_dist,value) VALUES (?, ?, ?, ?)",
                    id_transaction,
                    account_id,
                    account_dist,
                    value
                ).execute(&self.db).await.unwrap();

                sqlx::query!("INSERT INTO account_transaction(account_id,transaction_id) VALUES (?, ?)",
                    account_id,
                    id_transaction
                ).execute(&self.db).await.unwrap();

                sqlx::query!("INSERT INTO account_transaction(account_id,transaction_id) VALUES (?, ?)",
                    account_dist,
                    id_transaction
                ).execute(&self.db).await.unwrap();

                sqlx::query!("UPDATE account SET balance = ? WHERE id = ?", new_balance_dist, account_dist)
                    .execute(&self.db)
                    .await
                    .unwrap();

                sqlx::query!("UPDATE account SET balance = ? WHERE id = ?", remain, account_id)
                    .execute(&self.db)
                    .await
                    .unwrap();

                account.balance = remain;

                response = Ok(());
            }
            else {
                response = Err("Error: insufficient funds!");
            }

            let message= serde_json::to_string(&response).unwrap();
            self.write_message(message).await?;
        }
        Ok(())
    }

    async fn create_bill(&mut self, value: f32) -> io::Result<()> {
        let response: Result<Uuid, &str>;
        if let Some((u, a)) = &self.user {
            let bill_id = Uuid::new_v4();
            let bill_idh = bill_id.to_hyphenated();
            let account_id = a.id.to_hyphenated();

            sqlx::query!("INSERT INTO bill(id,account_id,favored_name,value) VALUES (?, ?, ?, ?)",
                bill_idh,
                account_id,
                u.name,
                value
            ).execute(&self.db).await.unwrap();

            response = Ok(bill_id);
        }
        else {
            response = Err("Error: You are not logged in!");
        }
        let message = serde_json::to_string(&response).unwrap();
        self.write_message(message).await
    }

    async fn get_basic_info(&mut self) -> io::Result<()> {
        let value: Option<(&str, f32)>;
        if let Some((u, a)) = &self.user {
            value = Some((&u.name, a.balance));
        } else {
            value = None;
        }
        let message = serde_json::to_string(&value).unwrap();
        self.write_message(message).await
    }

    async fn gen_statment(&mut self) -> io::Result<()> {
        let response: Result<String, &str>;
        if let Some((u, a)) = &self.user {
            let account_id = a.id.to_hyphenated();
            let mut statment = String::new();
            let records = sqlx::query!(
                "SELECT
                    account_transaction.account_id,
                    transactions.value,
                    transactions.data,
                    transactions.account_src,
                    transactions.account_dist,
                    user_src.name as name_src,
                    user_dist.name as name_dist
                FROM transactions
                INNER JOIN account_transaction ON account_transaction.transaction_id = transactions.id
                INNER JOIN user AS user_src ON user_src.account_id = transactions.account_src
                INNER JOIN user AS user_dist ON user_dist.account_id = transactions.account_dist
                WHERE account_transaction.account_id = ?",
                account_id
            ).fetch_all(&self.db).await.unwrap();
            for record in records {
                let line: String;
                if record.account_src == record.account_id {
                    line = format!("- ${} to {}, {}\n", record.value, record.name_dist, record.data);
                }
                else {
                    line = format!("+ ${} from {}, {}\n", record.value, record.name_src, record.data);
                }
                statment.push_str(&line);
            }
            response = Ok(statment);
        }
        else {
            response = Err("Error: You are not logged in!");
        }
        let message = serde_json::to_string(&response).unwrap();
        self.write_message(message).await
    }

    async fn logout(&mut self) {
        if self.user.is_some() {
            self.user = None;
        }
    }

    async fn close_connection(&mut self) {

    }
}