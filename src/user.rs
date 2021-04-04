use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
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
    pub birthdate: NaiveDateTime,
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
    pub birthdate: NaiveDateTime
}

#[derive(Serialize, Deserialize)]
pub enum UserAction {
    Login {cpf: String, password: String},
    Logout,
    CloseServerConnection,
    CreateUser(NewUser),
    DeleteUser,
    TransferMoney {dest_cpf: String, value: f32},
    GetBillInfo(Uuid),
    PayBill(Uuid),
    CreateBill(f32),
    GetStatment,
    GetBasicInfo
}