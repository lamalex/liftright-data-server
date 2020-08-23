use chrono::{offset::Utc, DateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{imu_record_pairs, imu_records};
use crate::LiftrightError;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImuRecordSet {
    pub session_id: Uuid,
    pub data: Vec<ImuRecordPair>,
}

impl ImuRecordSet {
    pub fn add(conn: &PgConnection, records: ImuRecordSet) -> Result<(), LiftrightError> {
        /*records.data.iter().map(|pair| {
            diesel::insert_into(imu_record_pairs::table)
            .values(pair)
            .execute(conn)
            .map_err(LiftrightError::DatabaseError)
        });*/

        Ok(())
    }
}