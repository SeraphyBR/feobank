use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub agency: u8,
    pub balance: f32,
    pub created_date: NaiveDateTime,
}
