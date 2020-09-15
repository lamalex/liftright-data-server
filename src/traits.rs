use crate::user::User;
use serde::Serialize;
use uuid::Uuid;

pub trait Sanitize {
    fn sanitize(self) -> String;
}
pub trait ExtractUser {
    fn extract_user(&self) -> User;
}

pub trait IdBucketPattern {
    fn to_bucket_selector(device_id: Uuid) -> Self;
}

pub trait BucketUpdate {
    fn to_bucket_update(
        field_name: &str,
        value: impl Serialize,
        to_increment_field_name: &str,
        increment_by: i32,
        id_prefix: &str,
    ) -> Self;
}

impl Sanitize for Uuid {
    fn sanitize(self) -> String {
        self.to_string().replace("-", "")
    }
}
