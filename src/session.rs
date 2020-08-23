use diesel::{insert_into, prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::sessions;
use crate::user::User;
use crate::LiftrightError;

#[derive(Debug, Clone, Queryable, Insertable, Associations, Serialize, Deserialize)]
#[belongs_to(User, foreign_key = "device_id")]
pub struct Session {
    pub id: Uuid,
    pub device_id: Uuid,
}

impl Session {
    pub fn get_or_make_if_new(
        conn: &PgConnection,
        session: Session,
    ) -> Result<Session, LiftrightError> {
        match Self::find_session(conn, &session)? {
            Some(session) => Ok(session),
            None => {
                Self::register_session(conn, &session)?;
                Self::get_or_make_if_new(conn, session)
            }
        }
    }

    fn register_session(conn: &PgConnection, session: &Session) -> Result<usize, LiftrightError> {
        insert_into(sessions::table)
            .values(session)
            .execute(conn)
            .map_err(LiftrightError::DatabaseError)
    }

    fn find_session(
        conn: &PgConnection,
        session: &Session,
    ) -> Result<Option<Session>, LiftrightError> {
        sessions::table
            .filter(sessions::id.eq(session.id))
            .first::<Session>(conn)
            .optional()
            .map_err(LiftrightError::DatabaseError)
    }
}
