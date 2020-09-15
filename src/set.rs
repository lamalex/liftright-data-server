use mongodb;
use mongodb::bson;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::{
    repetition::Repetition, session::Session, traits::IdBucketPattern, LrdsError, LrdsResult,
};
use uuid::Uuid;

const BUCKET_REP_LIMIT: i32 = 1000;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Set {
    #[serde(flatten)]
    session: Session,
    set_id: Uuid,
    level: String,
    exercise: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct SetQuery {
    id: bson::Bson,
    device_id: Uuid,
    session_id: Uuid,
    set_id: Uuid,
    rep_count: bson::Bson,
}

impl From<&Set> for SetQuery {
    fn from(set: &Set) -> Self {
        SetQuery {
            id: bson::Bson::to_bucket_selector(set.session.device_id),
            device_id: set.session.device_id,
            session_id: set.session.session_id,
            set_id: set.set_id,
            rep_count: bson::bson!({ "$lt": BUCKET_REP_LIMIT }),
        }
    }
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
            .map_err(LrdsError::DbError)?;
        Ok(())
    }
}
