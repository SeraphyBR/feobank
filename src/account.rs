use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Account {
    agency: u8,
    fullname: String,
    cpf: String,
    password: String,
    address: String,
    birthdate: NaiveDate,
    balance: Decimal,
    creation_date: NaiveDate,
    last_login: NaiveDate,
    last_login_device: String
}

#[derive(Serialize, Deserialize)]
pub struct NewAccount {
    fullname: String,
    cpf: String,
    password: String,
    address: String,
    birthdate: NaiveDate
}

impl Account {
    pub fn with(new: NewAccount) -> Account {
        todo!()
    }
}

#[derive(Serialize, Deserialize)]
pub enum AccountAction {
    Login {cpf: String, password: String},
    CreateAccount(NewAccount),
    DeleteAccount,
    TransferMoney {dest_cpf: String, value: Decimal},
    PayBill(Uuid),
    CreateBill {},
    GetStatment
}