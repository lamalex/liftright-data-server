use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{imurecords::ImuRecordPair, set::Set};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub level: String,
    pub sets: Vec<Set>,
    pub imu_records: Vec<ImuRecordPair>,
}
