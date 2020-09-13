use serde::{Deserialize, Serialize};

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
