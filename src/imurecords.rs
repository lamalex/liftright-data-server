use chrono::{offset::Utc, DateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{imu_pairs, imu_records};
use crate::sessions::Session;
use crate::LiftrightError;

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
pub struct ImuRecord {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImuRecordPair {
    pub acc: ImuRecord,
    pub gyro: ImuRecord,
}

#[derive(Debug, Clone, Queryable, Insertable, Identifiable)]
pub struct ImuPair {
    pub id: i32,
    pub session_id: Uuid,
    pub acc: i32,
    pub gyro: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImuRecordSet {
    pub session_id: Session,
    pub data: Vec<ImuRecordPair>,
}

pub fn add(_conn: &PgConnection, _data: ImuRecordSet) -> Result<(), LiftrightError> {
    Ok(())
}
