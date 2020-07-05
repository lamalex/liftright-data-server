use uuid::Uuid;
use diesel::prelude::*;
use chrono::{DateTime, offset::Utc};
use serde::{Serialize, Deserialize};

use crate::user::User;
use crate::LiftrightError;
use crate::schema::repetitions;

#[derive(Debug, Clone, Queryable, Identifiable, Associations, Serialize)]
#[belongs_to(User, foreign_key = "device_id")]
pub struct Repetition {
    pub id: i32,
    pub device_id: Uuid,
    pub session_id: Uuid,
    pub set_id: Uuid,
    pub exercise: String,
    pub rom: f32,
    pub velocity: f32,
    pub duration: f32,
    pub rep_time: DateTime<Utc>,
    pub level: String
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[table_name = "repetitions"]
pub struct NewRepetition {
    pub device_id: Uuid,
    pub session_id: Uuid,
    pub set_id: Uuid,
    pub exercise: String,
    pub rom: f32,
    pub velocity: f32,
    pub duration: f32,
    pub rep_time: DateTime<Utc>,
    pub level: String,
}

impl Repetition {
    pub fn create(conn: &PgConnection, new_rep: NewRepetition) -> Result<usize, LiftrightError> {
        crate::user::get_or_make_if_new(&conn, &new_rep.device_id)?;
        diesel::insert_into(repetitions::table)
            .values(&new_rep)
            .execute(conn)
            .map_err(LiftrightError::DatabaseError)
    }
}