use uuid::Uuid;
use diesel::prelude::*;
use chrono::{DateTime, offset::Utc};
use serde::{Serialize, Deserialize};

use crate::LiftrightError;
//use crate::repetition::Repetition;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImuRecord {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub time: DateTime<Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImuRecordPair {
    pub acc: ImuRecord,
    pub gyro: ImuRecord
}

#[derive(Debug, Clone, Serialize, Deserialize)]
//#[derive(Debug, Clone, Queryable, Identifiable, Associations, Serialize, Deserialize)]
    //#[belongs_to(Repetition, foreign_key = "set_id")]
pub struct ImuRecordSet {
    pub set_id: Uuid,
    pub data: Vec<ImuRecordPair>
}

pub fn add(_conn: &PgConnection, _data: ImuRecordSet) -> Result<(), LiftrightError> {
    Ok(())
}