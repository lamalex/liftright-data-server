use serde::{Deserialize, Serialize};
use uuid::Uuid;

//use crate::{LrdsResult, repetition::Repetition, set::Set};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Session {
    pub device_id: Uuid,
    pub session_id: Uuid,
}
