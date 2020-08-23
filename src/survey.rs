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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_survey_from_incoming() {
        let incoming = make_incoming_survey_from_valid();
        let survey = SurveyData::from(incoming);
        assert!(&survey.survey_data[..("ERROR PARSING MAP").len()] != "ERROR PARSING MAP")
    }

    #[test]
    fn error_on_missing_device_id() {
        let raw_json = format!(
            "
            {{
                \"device_id\": \"\",
                \"survey_data\": {}
            }}
            ",
            make_survey_data()
        );

        assert!(serde_json::from_str::<IncomingSurvey>(&raw_json).is_err())
    }

    #[test]
    fn error_on_missing_survey() {
        let raw_json = format!(
            "
            {{
                \"device_id\": \"{}\",
                \"survey_data\": \"\"
            }}
            ",
            Uuid::new_v4().to_string()
        );

        assert!(serde_json::from_str::<IncomingSurvey>(&raw_json).is_err())
    }

    fn make_incoming_survey_from_valid() -> IncomingSurvey {
        let raw_json = format!(
            "
            {{
                \"device_id\": \"{}\",
                \"survey_data\": {}  
            }}
            ",
            Uuid::new_v4().to_string(),
            make_survey_data()
        );

        serde_json::from_str(&raw_json).unwrap()
    }

    fn make_survey_data() -> String {
        let mut survey_data = HashMap::<String, Option<String>>::new();
        survey_data.insert(
            String::from("Was the game fun?"),
            Some(String::from("Very")),
        );
        survey_data.insert(
            String::from("Was the sensor/arm band comfortable?"),
            Some(String::from("Somewhat")),
        );
        survey_data.insert(
            String::from("What Metrics do you find useful to guage your performance over time?"),
            Some(String::from("I 💪🏻 my 🍆 and 🍑 into a 🐉")),
        );

        serde_json::to_string(&survey_data).unwrap()
    }
}
