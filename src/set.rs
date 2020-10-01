use mongodb;
use mongodb::bson;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::{
    query_selector::SetQuery, repetition::Repetition, session::Session, LrdsError, LrdsResult,
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Set {
    #[serde(flatten)]
    pub session: Session,
    pub set_id: Uuid,
    level: String,
    exercise: String,
}

impl TryFrom<&Set> for bson::Document {
    type Error = LrdsError;
    fn try_from(set: &Set) -> LrdsResult<Self> {
        bson::to_document(&SetQuery::from(set)).map_err(LrdsError::DbSerializationError)
    }
}

impl Set {
    pub async fn add_repetition(
        self,
        collection: mongodb::Collection,
        repetition: Repetition,
    ) -> LrdsResult<()> {
        // What about an enum with variants that we try_from instead of this kind of haphazard api
        let query = bson::Document::try_from(&self)?;
        let update = repetition.bucket_update(self.session);
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
