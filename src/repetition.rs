use uuid::Uuid;
use chrono::{DateTime, offset::Utc};

use crate::schema::repetitions;

#[derive(Queryable)]

pub struct Repetition {
    pub id: i32,
    pub device_id: Uuid,
    pub session_id: Uuid,
    pub set_id: Uuid,
    pub rom: f32,
    pub velocity: f32,
    pub duration: f32,
    pub rep_time: DateTime<Utc>,
    pub level: String
}