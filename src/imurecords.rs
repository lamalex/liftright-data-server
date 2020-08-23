use chrono::{offset::Utc, DateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{imu_pairs, imu_records};
use crate::LiftrightError;

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Serialize, Deserialize)]
pub struct ImuRecord {
    pub id: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub date: DateTime<Utc>,
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
    pub set_id: Uuid,
    pub data: Vec<ImuRecordPair>,
}

pub fn add(_conn: &PgConnection, _data: ImuRecordSet) -> Result<(), LiftrightError> {
    Ok(())
}
