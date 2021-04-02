use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub account_id: Uuid,
    pub cpf: String,
    pub password: String,
    pub email: String,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub birthdate: NaiveDate,
    pub last_login: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
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