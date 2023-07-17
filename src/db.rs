use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use super::models::{User, Invite};

type DBPool = Pool<ConnectionManager<PgConnection>>;

pub struct Repository {
    pool: DBPool,
}

impl Repository {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Repository { pool }
    }

    pub fn create_user(&self, new_user: User) -> Result<User, Error> {
        use super::schema::users::dsl::*;

        let connection = self.pool.get().expect("Failed to fetch a connection.");
        diesel::insert_into(users)
            .values(&new_user)
            .execute(&connection)?;

        Ok(new_user)
    }

    pub fn create_invite(&self, new_invite: Invite) -> Result<Invite, Error> {
        use super::schema::invites::dsl::*;

        let connection = self.pool.get().expect("Failed to fetch a connection.");
        diesel::insert_into(invites)
            .values(&new_invite)
            .execute(&connection)?;

        Ok(new_invite)
    }

    // Additional methods to fetch, update, delete users and invites...
}
