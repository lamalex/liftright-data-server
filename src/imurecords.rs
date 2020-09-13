use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::user::{ExtractUser, User};
use lrds_derive::ExtractUser;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ExtractUser)]
pub struct JsonImuRecordSet {
    device_id: Uuid,
    session_id: Uuid,
    data: Vec<ImuRecordPair>,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImuRecordPair {
    pub acc: ImuRecord,
    pub gyro: ImuRecord,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImuRecord {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub time: i64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImuDataUpdate {
    pub session_id: Uuid,
    pub data: Vec<ImuRecordPair>,
}

impl From<JsonImuRecordSet> for ImuDataUpdate {
    fn from(value: JsonImuRecordSet) -> Self {
        Self {
            session_id: value.session_id,
            data: value.data,
        }
    }
}
