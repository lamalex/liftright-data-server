use chrono::{offset::Utc, DateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::survey_results;
use crate::LiftrightError;

#[derive(Debug, Deserialize, Insertable, Serialize)]
#[table_name = "survey_results"]
pub struct Survey {
    pub device_id: Uuid,
    pub submitted: Option<DateTime<Utc>>,
    pub survey_data: serde_json::Value,
}

pub fn submit(conn: &PgConnection, data: Survey) -> Result<usize, LiftrightError> {
    diesel::insert_into(survey_results::table)
        .values(data)
        .execute(conn)
        .map_err(LiftrightError::DatabaseError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn deserialize_survey() {
        assert!(serde_json::from_str::<Survey>(&make_valid_survey_json()).is_ok());
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

        assert!(serde_json::from_str::<Survey>(&raw_json).is_err())
    }

    #[test]
    #[ignore]
    fn error_on_missing_survey() {
        let raw_json = format!(
            "
            {{
                \"device_id\": \"{}\",
                \"survey_data\": \"\"
            }}
            ",
            Uuid::new_v4()
        );

        assert!(serde_json::from_str::<Survey>(&raw_json).is_err())
    }

    fn make_valid_survey_json() -> String {
        format!(
            "
            {{
                \"device_id\": \"{}\",
                \"survey_data\": {}  
            }}
            ",
            Uuid::new_v4().to_string(),
            make_survey_data()
        )
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
