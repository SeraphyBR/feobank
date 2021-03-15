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
    creation_date: NaiveDate
}

#[derive(Serialize, Deserialize)]
pub enum AccountAction {
    Login {cpf: String, password: String},
    CreateAccount(),
    DeleteAccount,
    TransferMoney {dest_cpf: String, value: Decimal},
    PayBill(Uuid),
    CreateBill {},
    GetStatment
}

#[derive(Serialize, Deserialize)]
pub struct AccountActionResponse {
    action: AccountAction,
    response: String
}