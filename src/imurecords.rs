use chrono::{offset::Utc, DateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::session::Session;
use crate::schema::{imu_record_pairs, imu_records};
use crate::LiftrightError;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Insertable, Queryable)]
pub struct ImuRecord {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImuRecordSet {
    pub session_id: Uuid,
    pub data: Vec<ImuRecordPair>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ImuRecordPair {
    pub acc: ImuRecord,
    pub gyro: ImuRecord,
}

#[derive(Debug, Copy, Clone, Insertable, Queryable)]
#[table_name = "imu_records"]
struct NewImuRecord {
    pub id: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Copy, Clone, Insertable, Queryable, Associations)]
#[belongs_to(Session, foreign_key = "session_id")]
#[table_name = "imu_record_pairs"]
struct NewImuRecordPair {
    pub id: i32,
    pub session_id: Uuid,
    pub acc: i32,
    pub gyro: i32,  
}

#[derive(Debug, Copy, Clone, Insertable, Associations)]
#[belongs_to(Session, foreign_key = "session_id")]
#[table_name = "imu_record_pairs"]
struct InsertImuRecordPair {
    pub session_id: Uuid,
    pub acc: i32,
    pub gyro: i32,
}

impl ImuRecordSet {
    pub fn add(conn: &PgConnection, records: ImuRecordSet) -> Result<usize, LiftrightError> {
        use crate::schema::imu_records::dsl::*;
        use crate::schema::imu_record_pairs::dsl::*;
        let sess_id = records.session_id;

        let inserted_imu_data: Vec<NewImuRecord> = diesel::insert_into(imu_records)
            .values(records.data.iter()
                .map(|&pair| vec![pair.acc, pair.gyro])
                .flatten()
                .collect::<Vec<ImuRecord>>()
            )
            .get_results(conn).map_err(LiftrightError::DatabaseError)?;

        let pairs: Vec<InsertImuRecordPair> = inserted_imu_data.chunks(2).map(|pair| {
            InsertImuRecordPair {
                session_id: sess_id,
                acc: pair[0].id,
                gyro: pair[1].id
            }
        }).collect();

        diesel::insert_into(imu_record_pairs)
        .values(pairs)
        .execute(conn).map_err(LiftrightError::DatabaseError)
    }
}