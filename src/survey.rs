use chrono::{offset::Utc, DateTime};
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::schema::survey_results;
use crate::LiftrightError;

#[derive(Debug, Clone, Insertable, Deserialize)]
#[table_name = "survey_results"]
pub struct SurveyData {
    pub device_id: Uuid,
    pub submitted: Option<DateTime<Utc>>,
    pub survey_data: String,
}

pub fn submit(conn: &PgConnection, data: SurveyData) -> Result<usize, LiftrightError> {
    diesel::insert_into(survey_results::table)
        .values(&data)
        .execute(conn)
        .map_err(LiftrightError::DatabaseError)
}
