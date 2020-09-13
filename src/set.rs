use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::repetition::Repetition;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Set {
    pub id: Uuid,
    pub exercise: String,
    pub repetitions: Vec<Repetition>,
}
