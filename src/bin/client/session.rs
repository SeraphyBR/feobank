use uuid::Uuid;
use std::{io::{Read, Write}, net::{Shutdown, TcpStream}};

use feobank::{bill::Bill, user::*};

pub struct Session {
    conn: TcpStream
}

impl Session {
    pub fn new(conn: TcpStream) -> Session {
        Session {
            conn
        }
    }

    pub fn close(&mut self) {
        let action = UserAction::CloseServerConnection;
        let data = serde_json::to_string(&action).unwrap();
        self.write_message(data);
        self.conn.shutdown(Shutdown::Both).unwrap();
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
        let action = UserAction::Login {cpf, password};
        let data = serde_json::to_string(&action).unwrap();
        self.write_message(data);

        let response = self.read_message();
        serde_json::from_str(&response).unwrap()
    }

    pub fn logout(&mut self) {
        let action = UserAction::Logout;
        let data = serde_json::to_string(&action).unwrap();
        self.write_message(data);
    }

    pub fn create_user(&mut self, user: NewUser) -> Result<(), String> {
        let action = UserAction::CreateUser(user);
        let data = serde_json::to_string(&action).unwrap();
        self.write_message(data);
        let response = self.read_message();
        serde_json::from_str(&response).unwrap()
    }

    pub fn get_bill_info(&mut self, bill_id: Uuid) -> Result<Bill, String> {
        let action = UserAction::GetBillInfo(bill_id);
        let data = serde_json::to_string(&action).unwrap();
        self.write_message(data);
        let response = self.read_message();
        serde_json::from_str(&response).unwrap()
    }

    pub fn pay_bill(&mut self, bill_id: Uuid) -> Result<(), String> {
        let action = UserAction::PayBill(bill_id);
        let data = serde_json::to_string(&action).unwrap();
        self.write_message(data);
        let response = self.read_message();
        serde_json::from_str(&response).unwrap()
    }

    pub fn get_statment(&mut self) -> Result<String, String> {
        let action = UserAction::GetStatment;
        let data = serde_json::to_string(&action).unwrap();
        self.write_message(data);
        let response = self.read_message();
        serde_json::from_str(&response).unwrap()
    }

    pub fn create_bill(&mut self, value: f32) -> Result<Uuid, String> {
        let action = UserAction::CreateBill(value);
        let data = serde_json::to_string(&action).unwrap();
        self.write_message(data);
        let response = self.read_message();
        serde_json::from_str(&response).unwrap()
    }

    pub fn transfer_money(&mut self, dest_cpf: String, value: f32) -> Result<(), String> {
        let action = UserAction::TransferMoney {dest_cpf, value};
        let data = serde_json::to_string(&action).unwrap();
        self.write_message(data);
        let response = self.read_message();
        serde_json::from_str(&response).unwrap()
    }

    pub fn get_basic_info(&mut self) -> Option<(String, f32)> {
        let action = UserAction::GetBasicInfo;
        let data = serde_json::to_string(&action).unwrap();
        self.write_message(data);
        let response = self.read_message();
        serde_json::from_str(&response).unwrap()
    }
}