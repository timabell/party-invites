use diesel::prelude::*;
use crate::models::invite::{Invite, NewInvite};
use crate::schema::invites::dsl::*;

pub fn create_invite<'a>(conn: &PgConnection, name: &'a str, phone: &'a str) -> Invite {
    let new_invite = NewInvite {
        name,
        phone,
    };

    diesel::insert_into(invites)
        .values(&new_invite)
        .get_result(conn)
        .expect("Error saving new invite")
}

pub fn read_invites(conn: &PgConnection) -> Vec<Invite> {
    invites.load::<Invite>(conn).expect("Error loading invites")
}

pub fn update_invite(conn: &PgConnection, invite: &Invite) -> bool {
    let rows = diesel::update(invites.filter(id.eq(&invite.id)))
        .set(invite)
        .execute(conn)
        .expect("Error updating invite");

    rows > 0
}

pub fn delete_invite(conn: &PgConnection, invite_id: i32) -> bool {
    let rows = diesel::delete(invites.filter(id.eq(invite_id)))
        .execute(conn)
        .expect("Error deleting invite");

    rows > 0
}
