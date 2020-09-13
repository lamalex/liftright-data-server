use crate::user::{ExtractUser, User};
use lrds_derive::ExtractUser;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ExtractUser)]
pub struct JsonApiRepetition {
    device_id: Uuid,
    session_id: Uuid,
    set_id: Uuid,
    level: String,
    exercise: String,
    number: i32,
    rom: f64,
    duration: f64,
    time: i64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RepetitionUpdate {
    session_id: Uuid,
    set_id: Uuid,
    level: String,
    exercise: String,
    number: i32,
    rom: f64,
    duration: f64,
    time: i64,
}

impl From<JsonApiRepetition> for RepetitionUpdate {
    fn from(value: JsonApiRepetition) -> Self {
        Self {
            session_id: value.session_id,
            set_id: value.set_id,
            level: value.level,
            exercise: value.exercise,
            number: value.number,
            rom: value.rom,
            duration: value.duration,
            time: value.time,
        }
    }
}
