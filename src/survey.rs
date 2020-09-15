use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SurveyData {
    pub question: String,
    pub answer: Option<String>,
}
/*use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::traits::ExtractUser;
use crate::json_api::JsonSurvey;
use crate::user::User;
use lrds_derive::ExtractUser;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SurveyData {
    pub question: String,
    pub answer: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SurveyUpdate {
    pub submitted: DateTime<Utc>,
    pub survey_data: Vec<SurveyData>,
}

impl From<JsonSurvey> for SurveyUpdate {
    fn from(value: JsonSurvey) -> Self {
        Self {
            submitted: match value.submitted {
                Some(submitted) => submitted,
                None => Utc::now()
            },
            survey_data: value.survey_data
        }
    }
}
*/
