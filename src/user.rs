use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    account_id: Uuid,
    cpf: String,
    password: String,
    name: String,
    address: String,
    phone: String,
    birthdate: NaiveDate,
    last_login: NaiveDate,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub cpf: String,
    pub password: String,
    pub address: String,
    pub phone: String,
    pub birthdate: NaiveDate
}

#[derive(Serialize, Deserialize)]
pub enum UserAction {
    Login {cpf: String, password: String},
    CreateUser(NewUser),
    DeleteUser,
    TransferMoney {dest_cpf: String, value: Decimal},
    PayBill(Uuid),
    CreateBill {},
    GetStatment
}