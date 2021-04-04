use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Bill {
    pub id: Uuid,
    pub account_id: Uuid,
    pub favored_name: String,
    pub value: f32,
    pub created_date: NaiveDateTime
}