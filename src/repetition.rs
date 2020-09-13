use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Repetition {
    pub number: i32,
    pub rom: f64,
    pub duration: f64,
    pub velocity: f64,
    pub time: f64,
}
