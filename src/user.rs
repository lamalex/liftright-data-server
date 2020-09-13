use mongodb::{bson, options::FindOneOptions, Collection};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use uuid::Uuid;

use crate::{
    imurecords::{ImuDataUpdate, JsonImuRecordSet},
    repetition::{JsonApiRepetition, RepetitionUpdate},
    userquery::UserQuery,
    LiftrightError,
};

pub trait ExtractUser {
    fn extract_user(&self) -> User;
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RtfbStatusResult {
    rtfb_status: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct User {
    device_id: Uuid,
}

impl Into<UserQuery> for User {
    fn into(self) -> UserQuery {
        UserQuery::new(self.device_id)
    }
}

impl TryInto<bson::Document> for User {
    type Error = LiftrightError;

    fn try_into(self) -> Result<bson::Document, Self::Error> {
        let intermediate: UserQuery = self.into();
        intermediate.try_into()
    }
}

impl User {
    pub fn new(device_id: Uuid) -> Self {
        User { device_id }
    }

    async fn append_to_list<S: Serialize + From<I>, I>(
        self,
        collection: Collection,
        source: I,
        field_name: &str,
    ) -> Result<i64, LiftrightError> {
        let new_records =
            bson::to_bson(&S::from(source)).map_err(LiftrightError::DbSerializationError)?;

        self.update_user_record(
            collection,
            bson::doc! { "$push": { field_name: new_records } },
        )
        .await
    }

    async fn update_user_record(
        self,
        collection: Collection,
        document: bson::Document,
    ) -> Result<i64, LiftrightError> {
        let query = self.try_into()?;

        let update_options = mongodb::options::UpdateOptions::builder()
            .upsert(true)
            .build();

        let updated_res = collection
            .update_one(query, document, update_options)
            .await
            .map_err(LiftrightError::DbError)?;

        Ok(updated_res.modified_count)
    }

    pub async fn check_rtfb_status(self, collection: Collection) -> Result<bool, LiftrightError> {
        let filter: bson::Document = self.try_into()?;

        let rtfb_status_projection = FindOneOptions::builder()
            .projection(Some(bson::doc! {
                "rtfb_status": 1
            }))
            .build();

        let doc = collection
            .find_one(filter, rtfb_status_projection)
            .await
            .map_err(LiftrightError::DbError)?;

        match doc {
            Some(doc) => {
                let result: RtfbStatusResult =
                    bson::from_document(doc).map_err(LiftrightError::DbDeserializationError)?;
                Ok(result.rtfb_status)
            }
            None => Ok(false),
        }
    }

    pub async fn add_repetition(
        self,
        collection: Collection,
        repetition: JsonApiRepetition,
    ) -> Result<i64, LiftrightError> {
        let new_records = bson::to_bson(&RepetitionUpdate::from(repetition))
            .map_err(LiftrightError::DbSerializationError)?;
        self.update_user_record(
            collection,
            bson::doc! { "$push": { "repetitions": new_records } },
        )
        .await
    }

    pub async fn add_imu_records(
        self,
        collection: Collection,
        imurecords: JsonImuRecordSet,
    ) -> Result<i64, LiftrightError> {
        self.append_to_list::<ImuDataUpdate, _>(collection, imurecords, "imu_data")
            .await
    }
}
