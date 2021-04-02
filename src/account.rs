use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Account {
    id: Uuid,
    agency: u8,
    balance: Decimal,
    created_date: NaiveDate,
}
