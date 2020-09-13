use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::user::{ExtractUser, User};
use lrds_derive::ExtractUser;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ExtractUser)]
pub struct ImuRecordSet {
    device_id: Uuid,
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
    pub time: f64,
}
