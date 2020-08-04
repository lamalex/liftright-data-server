use chrono::{offset::Utc, DateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::schema::survey_results;
use crate::LiftrightError;

#[derive(Debug, Deserialize, Serialize)]
pub struct IncomingSurvey {
    pub device_id: Uuid,
    pub submitted: Option<DateTime<Utc>>,
    pub survey_data: HashMap<String, Option<String>>,
}
#[derive(Debug, Clone, Insertable, Deserialize, Serialize)]
#[table_name = "survey_results"]
pub struct SurveyData {
    pub device_id: Uuid,
    pub submitted: Option<DateTime<Utc>>,
    pub survey_data: String,
}

impl From<IncomingSurvey> for SurveyData {
    fn from(incoming: IncomingSurvey) -> Self {
        SurveyData {
            device_id: incoming.device_id,
            submitted: incoming.submitted,
            survey_data: match serde_json::to_string(&incoming.survey_data) {
                Ok(sd) => sd,
                Err(e) => format!("ERROR PARSING MAP, {}", e.to_string()),
            },
        }
    }
}

pub fn submit(conn: &PgConnection, data: IncomingSurvey) -> Result<usize, LiftrightError> {
    diesel::insert_into(survey_results::table)
        .values(SurveyData::from(data))
        .execute(conn)
        .map_err(LiftrightError::DatabaseError)
}
