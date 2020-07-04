use uuid::Uuid;
use chrono::{DateTime, offset::Utc};

#[derive(Queryable)]
pub struct Repetition {
    pub id: i32,
    pub device_id: Uuid,
    pub session_id: Uuid,
    pub set_id: Uuid,
    pub rom: f64,
    pub velocity: f64,
    pub duration: f64,
    pub rep_time: DateTime<Utc>,
    pub level: String
}