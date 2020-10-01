use crate::{
    session::Session,
    traits::{BucketUpdate, Sanitize},
};
use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Repetition {
    number: i32,
    rom: f64,
    duration: f64,
    time: i64,
}

impl Repetition {
    pub fn bucket_update(self, session: Session) -> bson::Document {
        bson::Document::to_bucket_update(
            "repetitions",
            self,
            "rep_count",
            1,
            &session.device_id.sanitize(),
        )
    }
}
