use uuid::Uuid;
use crate::schema::users;
use crate::LiftrightError;
use diesel::prelude::*;
use diesel::{self, insert_into};

#[derive(Queryable, Identifiable, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub device_id: Uuid,
    pub rtfb: bool
}

impl User {
    pub fn get_or_make_if_new(conn: &PgConnection, device_id: &Uuid) -> Result<User, LiftrightError> {
        match Self::find_user(conn, device_id)? {
            Some(user) => Ok(user),
            None => {
                Self::register_user(conn, device_id)?;
                Self::get_or_make_if_new(conn, device_id)
            }
        }
    }

    fn register_user(conn: &PgConnection, device_id: &Uuid) -> Result<usize, LiftrightError> {
        insert_into(users::table)
            .values(users::device_id.eq(device_id))
            .execute(conn)
            .map_err(LiftrightError::DatabaseError)
    }

    fn find_user(conn: &PgConnection, device_id: &Uuid) -> Result<Option<User>, LiftrightError> {
        let user = users::table
            .filter(users::device_id.eq(device_id))
            .first::<User>(conn)
            .optional()
            .map_err(LiftrightError::DatabaseError)?;

        Ok(user)
    }

    pub fn check_rtfb_status(conn: &PgConnection, device_id: &Uuid) -> Result<bool, LiftrightError> {
        let status = users::table
            .select(users::columns::rtfb)
            .filter(users::device_id.eq(device_id))
            .first::<bool>(conn)
            .map_err(LiftrightError::DatabaseError)?;

        Ok(status)
    }
}