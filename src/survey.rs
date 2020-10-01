use crate::{
    query_selector::{SurveyFilter, SurveyUpdate},
    LrdsError, LrdsResult,
};
use chrono::{DateTime, Utc};
use mongodb::bson;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Survey {
    pub device_id: Uuid,
    #[serde(default = "chrono::Utc::now")]
    pub submitted: DateTime<Utc>,
    #[serde(rename = "survey_data")]
    pub data: Vec<SurveyResponse>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SurveyResponse {
    pub question: String,
    pub answer: Option<String>,
}

enum Selector<'a> {
    Query(&'a Survey),
    Update(&'a Survey),
}

impl<'a> TryFrom<Selector<'a>> for bson::Document {
    type Error = LrdsError;
    fn try_from(var: Selector) -> LrdsResult<bson::Document> {
        use Selector::*;
        match var {
            Query(survey) => bson::to_document(&SurveyFilter::from(survey))
                .map_err(LrdsError::DbSerializationError),
            Update(survey) => bson::Document::try_from(&SurveyUpdate::from(survey)),
        }
    }
}

impl Survey {
    pub async fn insert(self, collection: mongodb::Collection) -> LrdsResult<()> {
        let query = bson::Document::try_from(Selector::Query(&self))?;
        let update = bson::Document::try_from(Selector::Update(&self))?;
        let options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();

        collection
            .update_one(query, update, options)
            .await
            .map_err(LrdsError::DbError)
            .map(|_| ())
    }
}
