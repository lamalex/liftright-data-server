use chrono::{offset::Utc, DateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::repetitions;
use crate::user::User;
use crate::LiftrightError;

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
    pub level: String,
}

#[derive(Debug, Clone, Insertable, Deserialize)]
#[table_name = "repetitions"]
pub struct NewRepetition {
    pub device_id: Uuid,
    pub session_id: Uuid,
    pub set_id: Uuid,
    pub exercise: String,
    pub number: i16,
    pub rom: f32,
    pub velocity: f32,
    pub duration: f32,
    pub rep_time: DateTime<Utc>,
    pub level: String,
}

impl Repetition {
    pub fn create(conn: &PgConnection, new_rep: NewRepetition) -> Result<usize, LiftrightError> {
        User::get_or_make_if_new(&conn, &new_rep.device_id)?;
        diesel::insert_into(repetitions::table)
            .values(&new_rep)
            .execute(conn)
            .map_err(LiftrightError::DatabaseError)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use uuid::Uuid;
    #[test]
    fn deserialize_repetiton() {
        serde_json::from_str::<NewRepetition>(&make_valid_rep_json_string()).unwrap();
    }

    fn make_valid_rep_json_string() -> String {
        format!(
            "
            {{
                \"device_id\": \"{}\",
                \"session_id\": \"{}\",
                \"set_id\": \"{}\",
                \"exercise\": \"BenchPress\",
                \"number\": 5,
                \"rom\": 48,
                \"velocity\": 66,
                \"duration\": 720,
                \"rep_time\": \"{}\",
                \"level\": \"MeetTheBar\"
            }}
        ",
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            DateTime::<Utc>::from(std::time::SystemTime::now())
        )
        /*let json: [String: Any] = [
            "device_id": deviceId.uuidString,
            "session_id": sessionId.uuidString,
            "set_id": setId.uuidString,
            "exercise": exercise.name,
            "number": rep.number,
            "rom": rep.liftingRom,
            "velocity": rep.liftingVelocity,
            "duration": rep.liftingMs,
            "rep_time": Self.dateFormatter.string(from: Date()),
            "level": level,
        ]*/
    }
}
