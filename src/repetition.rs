use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{user::User};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

impl JsonApiRepetition {
    pub fn extract_user(&self) -> User {
        User::new(self.device_id)
    }
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