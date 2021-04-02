use rust_decimal::Decimal;
use bcrypt::{DEFAULT_COST, hash, verify};
use std::{error::Error, io::{Read, Write}, net::{SocketAddr, TcpStream, Ipv4Addr}};

use feobank::user::*;
use feobank::user::UserAction::*;

pub struct Session {
    user: Option<User>,
    conn: TcpStream,
}

impl Session {
    pub fn new(conn: TcpStream) -> Session {
        Session {
            user: None,
            conn
        }
    }

    fn read_message(&mut self) -> String {
        let mut lenght = [0u8; std::mem::size_of::<usize>()];
        self.conn.read_exact(&mut lenght).unwrap();
        let lenght = usize::from_le_bytes(lenght);

        let mut buf = vec![0u8; lenght];
        self.conn.read_exact(&mut buf).unwrap();
        String::from_utf8_lossy(&buf).into()
    }

    fn write_message(&mut self, message: String) {
        let lenght = message.len().to_le_bytes();
        self.conn.write_all(&lenght).unwrap();
        self.conn.write_all(message.as_bytes()).unwrap();
    }

    pub fn login(&mut self, cpf: String, password: String) -> Result<(), String> {
        // Hash password with bcrypt
        let password = hash(password.as_str(), DEFAULT_COST).unwrap();

        let action = UserAction::Login {cpf, password};
        let data = serde_json::to_string(&action).unwrap();

        self.write_message(data);
        let response = self.read_message();
        self.user = serde_json::from_str(&response).unwrap();

        if self.user.is_none() {
            Err(self.read_message())
        }
        else {
            Ok(())
        }
    }

    pub fn create_user(&mut self, user: NewUser) -> Result<(), String> {
        let action = UserAction::CreateUser(user);
        let data = serde_json::to_string(&action).unwrap();
        self.write_message(data);
        let response = self.read_message();
        serde_json::from_str(&response).unwrap()
    }
}