use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{imurecords::ImuRecordSet, repetition::Repetition, set::Set, survey::SurveyData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddImuDataPayload {
    #[serde(flatten)]
    pub data: ImuRecordSet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRepetitionPayload {
    #[serde(flatten)]
    pub set: Set,
    #[serde(flatten)]
    pub repetition: Repetition,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddSurveyPayload {
    pub device_id: Uuid,
    pub submitted: Option<DateTime<Utc>>,
    pub survey_data: Vec<SurveyData>,
}
