use serde::{Deserialize, Serialize};

use crate::{imurecords::ImuRecordSet, repetition::Repetition, set::Set, survey::Survey};

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

pub type AddSurveyPayload = Survey;
