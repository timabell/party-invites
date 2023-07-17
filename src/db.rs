use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use super::models::User;

type DBPool = Pool<ConnectionManager<PgConnection>>;

pub struct UserRepository {
    pool: DBPool,
}

impl UserRepository {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        UserRepository { pool }
    }

    pub fn create_user(&self, new_user: User) -> Result<User, Error> {
        use super::schema::users::dsl::*;

        let connection = self.pool.get().expect("Failed to fetch a connection.");
        diesel::insert_into(users)
            .values(&new_user)
            .execute(&connection)?;

        Ok(new_user)
    }

    // Additional methods to fetch, update, delete users...
}
