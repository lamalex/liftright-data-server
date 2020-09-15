use crate::traits::{BucketUpdate, IdBucketPattern, Sanitize};
use chrono;
use mongodb::bson;
use serde::Serialize;
use uuid::Uuid;

impl IdBucketPattern for bson::Bson {
    fn to_bucket_selector(device_id: Uuid) -> Self {
        let id = format!("^{}_", device_id.sanitize());
        let re = mongodb::bson::Regex {
            pattern: id,
            options: String::new(),
        };

        bson::Bson::RegularExpression(re)
    }
}

impl BucketUpdate for bson::Document {
    fn to_bucket_update(
        field_name: &str,
        value: impl Serialize,
        to_increment_field_name: &str,
        increment_by: i32,
        id_prefix: &str,
    ) -> Self {
        let now = chrono::Utc::now().timestamp_nanos();

        bson::doc! {
            "$push": {
                field_name: bson::to_bson(&value).unwrap(),
            },
            "$inc": { to_increment_field_name: increment_by },
            "$setOnInsert": { "id": format!("{}_{}", id_prefix, now) }
        }
    }
}
